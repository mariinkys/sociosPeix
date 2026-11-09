//! Minimal in-process job scheduler: "run this every day at HH:MM local time"

use std::{future::Future, time::Duration};

use chrono::{DateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Tz;
use tracing::{Instrument, info, info_span};

/// Next occurrence of `at` (local wall-clock time in `tz`) strictly after `now`.
/// On DST gaps the earliest valid instant is used; ambiguous times run once.
pub fn next_run(now: DateTime<Utc>, at: NaiveTime, tz: Tz) -> DateTime<Utc> {
    let local_today = now.with_timezone(&tz).date_naive();
    for offset in 0..=2 {
        let date = local_today + chrono::Days::new(offset);
        let candidate = tz
            .from_local_datetime(&date.and_time(at))
            .earliest()
            // Wall-clock time skipped by DST: run an hour later instead.
            .or_else(|| {
                tz.from_local_datetime(&(date.and_time(at) + chrono::Duration::hours(1)))
                    .earliest()
            });
        if let Some(c) = candidate.map(|c| c.with_timezone(&Utc))
            && c > now
        {
            return c;
        }
    }
    now + chrono::Duration::days(1)
}

/// Spawns a task that runs `job` every day at `at` in `tz`.
pub fn spawn_daily<F, Fut>(name: &'static str, at: NaiveTime, tz: Tz, job: F)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            let now = Utc::now();
            let next = next_run(now, at, tz);
            info!(job = name, next_run = %next.with_timezone(&tz), "scheduled");
            let wait = (next - now).to_std().unwrap_or(Duration::from_secs(1));
            tokio::time::sleep(wait).await;
            job().instrument(info_span!("job", name)).await;
        }
    });
}

/// Spawns a task that runs `job` every `period` (first run after one period).
pub fn spawn_every<F, Fut>(period: Duration, job: F)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        let mut tick = tokio::time::interval_at(tokio::time::Instant::now() + period, period);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            job().await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utc(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    #[test]
    fn schedules_today_or_tomorrow_in_local_time() {
        let tz = chrono_tz::Europe::Madrid;
        let eight = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        // 06:00 UTC in winter = 07:00 Madrid → today 08:00 Madrid = 07:00 UTC.
        assert_eq!(
            next_run(utc("2026-01-10T06:00:00Z"), eight, tz),
            utc("2026-01-10T07:00:00Z")
        );
        // Just after → tomorrow.
        assert_eq!(
            next_run(utc("2026-01-10T07:00:01Z"), eight, tz),
            utc("2026-01-11T07:00:00Z")
        );
        // Summer time: 08:00 Madrid = 06:00 UTC.
        assert_eq!(
            next_run(utc("2026-07-10T05:00:00Z"), eight, tz),
            utc("2026-07-10T06:00:00Z")
        );
    }

    #[test]
    fn handles_dst_gap() {
        // 2026-03-29 02:30 doesn't exist in Madrid (clocks jump 02:00 → 03:00).
        let tz = chrono_tz::Europe::Madrid;
        let t = NaiveTime::from_hms_opt(2, 30, 0).unwrap();
        let next = next_run(utc("2026-03-28T23:00:00Z"), t, tz);
        assert_eq!(next, utc("2026-03-29T01:30:00Z")); // 03:30 local
    }
}
