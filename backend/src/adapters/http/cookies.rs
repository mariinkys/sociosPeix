use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;

pub const ACCESS_TOKEN: &str = "access_token";
pub const REFRESH_TOKEN: &str = "refresh_token";
/// The refresh token is only ever sent to this path.
const REFRESH_PATH: &str = "/api/auth/refresh";

/// Builds the HttpOnly, SameSite=Strict session cookies.
pub struct CookieSettings {
    pub secure: bool,
    pub access_ttl: Duration,
    pub refresh_ttl: Duration,
}

impl CookieSettings {
    fn build(
        &self,
        name: &'static str,
        value: String,
        path: &'static str,
        max_age: Duration,
    ) -> Cookie<'static> {
        Cookie::build((name, value))
            .http_only(true)
            .secure(self.secure)
            .same_site(SameSite::Strict)
            .path(path)
            .max_age(max_age)
            .build()
    }

    pub fn access(&self, token: String) -> Cookie<'static> {
        self.build(ACCESS_TOKEN, token, "/", self.access_ttl)
    }

    pub fn refresh(&self, token: String) -> Cookie<'static> {
        self.build(REFRESH_TOKEN, token, REFRESH_PATH, self.refresh_ttl)
    }

    pub fn clear_access(&self) -> Cookie<'static> {
        self.build(ACCESS_TOKEN, String::new(), "/", Duration::ZERO)
    }

    pub fn clear_refresh(&self) -> Cookie<'static> {
        self.build(REFRESH_TOKEN, String::new(), REFRESH_PATH, Duration::ZERO)
    }
}
