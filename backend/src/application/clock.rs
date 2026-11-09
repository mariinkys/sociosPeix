use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;

/// Time source for the application.
///
/// Timestamps are stored as naive **UTC** in the database (`TIMESTAMP` columns),
/// while calendar questions — "whose birthday is today?", "how many emails were
/// sent today?" — are answered in the hotel's local time zone.
#[derive(Debug, Clone, Copy)]
pub struct Clock {
    tz: Tz,
}

impl Clock {
    pub fn new(tz: Tz) -> Self {
        Self { tz }
    }

    pub fn tz(&self) -> Tz {
        self.tz
    }

    /// Current instant as naive UTC (the representation used in the DB).
    pub fn now(&self) -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    pub fn now_local(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&self.tz)
    }

    /// Today's date in the local time zone.
    pub fn today(&self) -> NaiveDate {
        self.now_local().date_naive()
    }

    /// Local midnight of today, expressed as naive UTC.
    pub fn start_of_today_utc(&self) -> NaiveDateTime {
        self.start_of_day_utc(self.today())
    }

    pub fn start_of_day_utc(&self, date: NaiveDate) -> NaiveDateTime {
        let midnight = date.and_hms_opt(0, 0, 0).expect("midnight is valid");
        // On a DST change at midnight the local time may be skipped/ambiguous;
        // take the earliest valid instant, falling back to treating it as UTC.
        self.tz
            .from_local_datetime(&midnight)
            .earliest()
            .map(|dt| dt.naive_utc())
            .unwrap_or(midnight)
    }

    /// Day and month of today, for birthday lookups.
    pub fn today_day_month(&self) -> (u32, u32) {
        let today = self.today();
        (today.day(), today.month())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_day_is_converted_to_utc() {
        let clock = Clock::new(chrono_tz::Europe::Madrid);
        let winter = NaiveDate::from_ymd_opt(2026, 1, 10).unwrap();
        assert_eq!(
            clock.start_of_day_utc(winter).to_string(),
            "2026-01-09 23:00:00"
        );
        let summer = NaiveDate::from_ymd_opt(2026, 7, 10).unwrap();
        assert_eq!(
            clock.start_of_day_utc(summer).to_string(),
            "2026-07-09 22:00:00"
        );
    }
}
