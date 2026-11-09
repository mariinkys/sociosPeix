//! Composition root: builds every adapter and use case and wires them together.

use std::{sync::Arc, time::Duration};

use bytes::Bytes;
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        crypto::{bcrypt::BcryptPasswordHasher, jwt::JwtTokenIssuer},
        email::{
            Sender, http_client,
            mailtrap::{self, MailtrapSender},
            resend::{self, ResendSender},
        },
        excel::XlsxExporter,
        http::{app_state::AppState, cookies::CookieSettings, rate_limit::RateLimiter},
        ocr::DisabledFormAnalyzer,
        persistence::PostgresPersistence,
    },
    clock::Clock,
    infra::{config::AppConfig, db::init_db, scheduler},
    use_cases::{
        auth::{AuthSettings, AuthUseCases},
        birthday::BirthdayUseCases,
        catalog::CatalogUseCases,
        email::{EmailSender, EmailUseCases},
        form_ocr::{FormAnalyzer, FormOcrUseCases},
        interest::InterestUseCases,
        member::MemberUseCases,
        password_reset::PasswordResetUseCases,
        statistics::StatisticsUseCases,
        user::{PasswordHasher, UserUseCases},
    },
};

/// The hotel logo embedded in birthday emails (compiled into the binary).
static LOGO_PNG: &[u8] = include_bytes!("../../assets/logo.png");

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,tower_http=info"));
    let json = std::env::var("LOG_FORMAT").is_ok_and(|v| v.eq_ignore_ascii_case("json"));
    let registry = tracing_subscriber::registry().with(filter);
    let result = if json {
        registry
            .with(fmt::layer().json().with_current_span(false))
            .try_init()
    } else {
        registry.with(fmt::layer().with_target(false)).try_init()
    };
    result.ok();
}

/// Background jobs that used to be Spring `@Scheduled` methods.
pub struct Jobs {
    pub auth: Arc<AuthUseCases>,
    pub password_reset: Arc<PasswordResetUseCases>,
    pub birthdays: Option<Arc<BirthdayUseCases>>,
    pub rate_limiter: Arc<RateLimiter>,
}

pub async fn init_app_state(config: &AppConfig) -> anyhow::Result<(AppState, Jobs)> {
    let clock = Clock::new(config.timezone);
    let pool = init_db(config).await?;
    let db = Arc::new(PostgresPersistence::new(pool));

    let hasher: Arc<dyn PasswordHasher> = Arc::new(BcryptPasswordHasher);
    let jwt = Arc::new(JwtTokenIssuer::new(
        config.jwt_secret.as_bytes(),
        config.access_token_ttl.as_secs(),
    )?);
    let exporter = Arc::new(XlsxExporter);

    let auth = Arc::new(AuthUseCases::new(
        db.clone(),
        hasher.clone(),
        db.clone(),
        jwt,
        AuthSettings {
            max_login_attempts: config.max_login_attempts,
            lockout_duration: chrono::Duration::from_std(config.lockout_duration)?,
            refresh_token_ttl: chrono::Duration::from_std(config.refresh_token_ttl)?,
        },
        clock,
        hasher.hash("timing-equaliser").await?,
    ));
    let users = Arc::new(UserUseCases::new(db.clone(), hasher.clone()));
    let catalog = Arc::new(CatalogUseCases::new(db.clone()));
    let interests = Arc::new(InterestUseCases::new(db.clone()));
    let members = Arc::new(MemberUseCases::new(
        db.clone(),
        catalog.clone(),
        exporter.clone(),
        clock,
    ));
    let statistics = Arc::new(StatisticsUseCases::new(db.clone(), exporter));

    let emails = Arc::new(EmailUseCases::new(
        db.clone(),
        db.clone(),
        db.clone(),
        email_providers(config)?,
        clock,
    ));
    let password_reset = Arc::new(PasswordResetUseCases::new(
        db.clone(),
        db.clone(),
        hasher,
        db.clone(),
        emails.clone(),
        clock,
    ));
    let birthdays = config.birthday_emails_enabled.then(|| {
        Arc::new(BirthdayUseCases::new(
            db.clone(),
            emails.clone(),
            clock,
            Bytes::from_static(LOGO_PNG),
        ))
    });
    let form_ocr = Arc::new(FormOcrUseCases::new(form_analyzer(config)?));

    let rate_limiter = Arc::new(RateLimiter::new(
        config.login_rate_limit_per_minute,
        Duration::from_secs(60),
        config.trust_proxy,
    ));

    let state = AppState {
        cookies: Arc::new(CookieSettings {
            secure: config.cookie_secure,
            access_ttl: time::Duration::seconds(config.access_token_ttl.as_secs() as i64),
            refresh_ttl: time::Duration::seconds(config.refresh_token_ttl.as_secs() as i64),
        }),
        login_rate_limiter: rate_limiter.clone(),
        auth: auth.clone(),
        users,
        password_reset: password_reset.clone(),
        catalog,
        interests,
        members,
        emails,
        statistics,
        form_ocr,
    };
    let jobs = Jobs {
        auth,
        password_reset,
        birthdays,
        rate_limiter,
    };
    Ok((state, jobs))
}

fn email_providers(config: &AppConfig) -> anyhow::Result<Vec<Arc<dyn EmailSender>>> {
    let client = http_client()?;
    let from = Sender::parse(&config.email_from);
    let mut providers: Vec<Arc<dyn EmailSender>> = Vec::new();
    if let Some(c) = &config.resend {
        providers.push(Arc::new(ResendSender::new(
            client.clone(),
            c.api_url
                .clone()
                .unwrap_or_else(|| resend::DEFAULT_ENDPOINT.into()),
            c.api_key.clone(),
            from.clone(),
            c.daily_limit,
        )));
    }
    if let Some(c) = &config.mailtrap {
        providers.push(Arc::new(MailtrapSender::new(
            client,
            c.api_url
                .clone()
                .unwrap_or_else(|| mailtrap::DEFAULT_ENDPOINT.into()),
            c.api_key.clone(),
            from,
            c.daily_limit,
        )));
    }
    let names: Vec<&str> = providers.iter().map(|p| p.name()).collect();
    info!(providers = ?names, "email providers configured");
    Ok(providers)
}

#[cfg(feature = "ocr")]
fn form_analyzer(config: &AppConfig) -> anyhow::Result<Arc<dyn FormAnalyzer>> {
    use crate::adapters::ocr::OnnxFormAnalyzer;
    use pdf_analyzer::config::FormTemplate;

    if !config.ocr_enabled {
        info!("form OCR disabled by configuration");
        return Ok(Arc::new(DisabledFormAnalyzer));
    }
    let template = match &config.ocr_template_path {
        Some(path) => FormTemplate::load(std::path::Path::new(path))?,
        None => FormTemplate::parse(pdf_analyzer::BUNDLED_TEMPLATE)?,
    };
    let analyzer = OnnxFormAnalyzer::new(template, config.ocr_idle_unload);
    if config.ocr_preload {
        analyzer.preload();
    }
    info!(idle_unload = ?config.ocr_idle_unload, "form OCR enabled (models load on first use)");
    Ok(Arc::new(analyzer))
}

#[cfg(not(feature = "ocr"))]
fn form_analyzer(config: &AppConfig) -> anyhow::Result<Arc<dyn FormAnalyzer>> {
    if config.ocr_enabled {
        warn!("OCR_ENABLED is set but this binary was built without the `ocr` feature");
    }
    Ok(Arc::new(DisabledFormAnalyzer))
}

/// Starts the daily maintenance and birthday jobs.
pub fn spawn_jobs(config: &AppConfig, jobs: Jobs) {
    let tz = config.timezone;
    let at = |h, m| chrono::NaiveTime::from_hms_opt(h, m, 0).expect("valid time");

    let auth = jobs.auth;
    scheduler::spawn_daily("refresh-token-cleanup", at(3, 0), tz, move || {
        let auth = auth.clone();
        async move {
            match auth.delete_expired_tokens().await {
                Ok(n) => info!(deleted = n, "expired refresh tokens removed"),
                Err(e) => error!(error = %e, "refresh token cleanup failed"),
            }
        }
    });

    let reset = jobs.password_reset;
    scheduler::spawn_daily("reset-code-cleanup", at(3, 30), tz, move || {
        let reset = reset.clone();
        async move {
            match reset.delete_expired_codes().await {
                Ok(n) => info!(deleted = n, "expired password reset codes removed"),
                Err(e) => error!(error = %e, "reset code cleanup failed"),
            }
        }
    });

    if let Some(birthdays) = jobs.birthdays {
        scheduler::spawn_daily(
            "birthday-emails",
            config.birthday_email_time,
            tz,
            move || {
                let birthdays = birthdays.clone();
                async move {
                    if let Err(e) = birthdays.send_birthday_emails().await {
                        error!(error = %e, "birthday job failed");
                    }
                }
            },
        );
    } else {
        warn!("birthday emails are disabled");
    }

    let limiter = jobs.rate_limiter;
    scheduler::spawn_every(Duration::from_secs(5 * 60), move || {
        let limiter = limiter.clone();
        async move { limiter.evict_idle() }
    });
}
