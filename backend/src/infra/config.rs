use std::{env, net::SocketAddr, str::FromStr, time::Duration};

use anyhow::{Context, bail};
use chrono::NaiveTime;
use chrono_tz::Tz;

/// Where to find PostgreSQL. Either a full `DATABASE_URL`, or discrete
/// parts (which avoids URL-escaping problems with special characters in
/// passwords).
#[derive(Clone)]
pub enum DatabaseConfig {
    Url(String),
    Parts {
        host: String,
        port: u16,
        name: String,
        username: String,
        password: String,
    },
}

#[derive(Clone)]
pub struct EmailProviderConfig {
    pub api_key: String,
    pub daily_limit: u32,
    /// Override of the provider's API URL (tests / sandboxes).
    pub api_url: Option<String>,
}

/// All runtime configuration, read once from the environment at startup.
/// Deliberately not `Debug`: it holds secrets.
#[derive(Clone)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub database: DatabaseConfig,
    pub db_max_connections: u32,

    pub jwt_secret: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub cookie_secure: bool,

    pub cors_allowed_origins: Vec<String>,
    pub trust_proxy: bool,
    pub login_rate_limit_per_minute: u32,
    pub max_login_attempts: i32,
    pub lockout_duration: Duration,
    pub max_upload_bytes: usize,

    pub timezone: Tz,
    pub email_from: String,
    pub resend: Option<EmailProviderConfig>,
    pub mailtrap: Option<EmailProviderConfig>,
    pub birthday_emails_enabled: bool,
    pub birthday_email_time: NaiveTime,

    pub ocr_enabled: bool,
    pub ocr_template_path: Option<String>,
    pub ocr_idle_unload: Option<Duration>,
    pub ocr_preload: bool,
}

fn var(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
}

fn parse_or<T: FromStr>(key: &str, default: T) -> anyhow::Result<T>
where
    T::Err: std::fmt::Display,
{
    match var(key) {
        None => Ok(default),
        Some(v) => v
            .parse()
            .map_err(|e| anyhow::anyhow!("{key}={v:?} is invalid: {e}")),
    }
}

fn flag(key: &str, default: bool) -> anyhow::Result<bool> {
    match var(key).map(|v| v.to_ascii_lowercase()) {
        None => Ok(default),
        Some(v) if matches!(v.as_str(), "1" | "true" | "yes" | "on") => Ok(true),
        Some(v) if matches!(v.as_str(), "0" | "false" | "no" | "off") => Ok(false),
        Some(v) => bail!("{key}={v:?} is not a boolean"),
    }
}

/// A provider is enabled only when it has a plausible API key.
fn email_provider(prefix: &str, default_limit: u32) -> anyhow::Result<Option<EmailProviderConfig>> {
    let daily_limit = parse_or(&format!("{prefix}_DAILY_LIMIT"), default_limit)?;
    let api_url = var(&format!("{prefix}_API_URL"));
    Ok(var(&format!("{prefix}_API_KEY"))
        .filter(|k| k.len() > 5)
        .map(|api_key| EmailProviderConfig {
            api_key,
            daily_limit,
            api_url,
        }))
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let database = match var("DATABASE_URL") {
            Some(url) => DatabaseConfig::Url(url),
            None => DatabaseConfig::Parts {
                host: var("DB_HOST").unwrap_or_else(|| "localhost".into()),
                port: parse_or("DB_PORT", 5432)?,
                name: var("DB_NAME").unwrap_or_else(|| "sociosPeix".into()),
                username: var("DB_USERNAME")
                    .context("DB_USERNAME (or DATABASE_URL) must be set")?,
                password: var("DB_PASSWORD")
                    .context("DB_PASSWORD (or DATABASE_URL) must be set")?,
            },
        };

        let jwt_secret = var("JWT_SECRET").context("JWT_SECRET must be set")?;

        let birthday_email_time = NaiveTime::parse_from_str(
            &var("BIRTHDAY_EMAIL_TIME").unwrap_or_else(|| "08:00".into()),
            "%H:%M",
        )
        .context("BIRTHDAY_EMAIL_TIME must look like HH:MM")?;

        let timezone: Tz = var("APP_TIMEZONE")
            .unwrap_or_else(|| "Europe/Madrid".into())
            .parse()
            .map_err(|e| anyhow::anyhow!("APP_TIMEZONE is not a valid IANA zone: {e}"))?;

        let idle_minutes: u64 = parse_or("OCR_IDLE_UNLOAD_MINUTES", 15)?;

        Ok(Self {
            bind_addr: parse_or("BIND_ADDR", SocketAddr::from(([0, 0, 0, 0], 8080)))?,
            database,
            db_max_connections: parse_or("DB_MAX_CONNECTIONS", 5)?,

            jwt_secret,
            access_token_ttl: Duration::from_secs(parse_or("ACCESS_TOKEN_TTL_SECS", 3 * 60 * 60)?),
            refresh_token_ttl: Duration::from_secs(
                parse_or::<u64>("REFRESH_TOKEN_TTL_DAYS", 7)? * 24 * 60 * 60,
            ),
            cookie_secure: flag("COOKIE_SECURE", false)?,

            cors_allowed_origins: var("CORS_ALLOWED_ORIGIN")
                .unwrap_or_else(|| "http://localhost:5173".into())
                .split(',')
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            trust_proxy: flag("TRUST_PROXY", false)?,
            login_rate_limit_per_minute: parse_or("LOGIN_RATE_LIMIT_PER_MINUTE", 5)?,
            max_login_attempts: parse_or("MAX_LOGIN_ATTEMPTS", 5)?,
            lockout_duration: Duration::from_secs(
                parse_or::<u64>("LOCKOUT_DURATION_MINUTES", 15)? * 60,
            ),
            max_upload_bytes: parse_or::<usize>("MAX_UPLOAD_MB", 25)? * 1024 * 1024,

            timezone,
            email_from: var("EMAIL_FROM").unwrap_or_else(|| "onboarding@resend.dev".into()),
            resend: email_provider("RESEND", 100)?,
            mailtrap: email_provider("MAILTRAP", 150)?,
            birthday_emails_enabled: flag("BIRTHDAY_EMAILS_ENABLED", true)?,
            birthday_email_time,

            ocr_enabled: flag("OCR_ENABLED", true)?,
            ocr_template_path: var("OCR_TEMPLATE_PATH"),
            ocr_idle_unload: (idle_minutes > 0).then(|| Duration::from_secs(idle_minutes * 60)),
            ocr_preload: flag("OCR_PRELOAD", false)?,
        })
    }
}
