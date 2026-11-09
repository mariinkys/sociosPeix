//! Per-client token-bucket rate limiting for the login/register endpoints.

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, Method, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::adapters::http::app_error_impl::error_response;

struct Bucket {
    tokens: f64,
    updated: Instant,
}

/// `capacity` requests per `period`, refilled continuously.
pub struct RateLimiter {
    capacity: f64,
    refill_per_sec: f64,
    trust_proxy: bool,
    buckets: Mutex<HashMap<IpAddr, Bucket>>,
}

impl RateLimiter {
    pub fn new(capacity: u32, period: Duration, trust_proxy: bool) -> Self {
        let capacity = f64::from(capacity.max(1));
        Self {
            capacity,
            refill_per_sec: capacity / period.as_secs_f64(),
            trust_proxy,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn try_acquire(&self, ip: IpAddr) -> bool {
        self.try_acquire_at(ip, Instant::now())
    }

    fn try_acquire_at(&self, ip: IpAddr, now: Instant) -> bool {
        let mut buckets = self.buckets.lock().unwrap_or_else(|p| p.into_inner());
        let bucket = buckets.entry(ip).or_insert(Bucket {
            tokens: self.capacity,
            updated: now,
        });
        let elapsed = now.saturating_duration_since(bucket.updated).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * self.refill_per_sec).min(self.capacity);
        bucket.updated = now;
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Forgets clients whose bucket has fully refilled, so memory stays
    /// bounded by the number of *recent* clients (the old implementation kept
    /// every IP it had ever seen).
    pub fn evict_idle(&self) {
        let now = Instant::now();
        let full_after = Duration::from_secs_f64(self.capacity / self.refill_per_sec);
        let mut buckets = self.buckets.lock().unwrap_or_else(|p| p.into_inner());
        buckets.retain(|_, b| now.saturating_duration_since(b.updated) < full_after);
        buckets.shrink_to_fit();
    }

    /// The client address. Behind our own reverse proxy (`TRUST_PROXY=true`)
    /// it's taken from `X-Real-IP`, or else the *last* `X-Forwarded-For` hop —
    /// the one appended by the proxy, which the client can't forge.
    pub fn client_ip(&self, headers: &HeaderMap, peer: IpAddr) -> IpAddr {
        if !self.trust_proxy {
            return peer;
        }
        let header = |name| headers.get(name).and_then(|v| v.to_str().ok());
        header("x-real-ip")
            .and_then(|v| v.trim().parse().ok())
            .or_else(|| {
                header("x-forwarded-for")
                    .and_then(|v| v.rsplit(',').next())
                    .and_then(|v| v.trim().parse().ok())
            })
            .unwrap_or(peer)
    }
}

pub async fn limit_logins(
    State(limiter): State<Arc<RateLimiter>>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    // CORS preflights must never consume tokens.
    if req.method() == Method::OPTIONS {
        return next.run(req).await;
    }
    let ip = limiter.client_ip(req.headers(), peer.ip());
    if limiter.try_acquire(ip) {
        next.run(req).await
    } else {
        tracing::warn!(%ip, "login rate limit exceeded");
        error_response(
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests, please try again later.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_allows_burst_then_refills() {
        let rl = RateLimiter::new(5, Duration::from_secs(60), false);
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        let t0 = Instant::now();
        for _ in 0..5 {
            assert!(rl.try_acquire_at(ip, t0));
        }
        assert!(!rl.try_acquire_at(ip, t0));
        // One token every 12 s.
        assert!(rl.try_acquire_at(ip, t0 + Duration::from_secs(12)));
        assert!(!rl.try_acquire_at(ip, t0 + Duration::from_secs(13)));
        // Other clients are independent.
        assert!(rl.try_acquire_at("10.0.0.2".parse().unwrap(), t0));
    }

    #[test]
    fn proxy_headers_only_when_trusted() {
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", "6.6.6.6, 1.2.3.4".parse().unwrap());
        let peer: IpAddr = "172.18.0.5".parse().unwrap();
        assert_eq!(
            RateLimiter::new(5, Duration::from_secs(60), false).client_ip(&h, peer),
            peer
        );
        let trusted = RateLimiter::new(5, Duration::from_secs(60), true);
        assert_eq!(
            trusted.client_ip(&h, peer),
            "1.2.3.4".parse::<IpAddr>().unwrap()
        );
        h.insert("x-real-ip", "9.9.9.9".parse().unwrap());
        assert_eq!(
            trusted.client_ip(&h, peer),
            "9.9.9.9".parse::<IpAddr>().unwrap()
        );
    }
}
