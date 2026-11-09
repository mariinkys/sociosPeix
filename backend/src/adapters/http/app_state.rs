use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    adapters::http::{cookies::CookieSettings, rate_limit::RateLimiter},
    use_cases::{
        auth::AuthUseCases, catalog::CatalogUseCases, email::EmailUseCases,
        form_ocr::FormOcrUseCases, interest::InterestUseCases, member::MemberUseCases,
        password_reset::PasswordResetUseCases, statistics::StatisticsUseCases, user::UserUseCases,
    },
};

/// Everything a handler may need
#[derive(Clone)]
pub struct AppState {
    pub cookies: Arc<CookieSettings>,
    pub login_rate_limiter: Arc<RateLimiter>,
    pub auth: Arc<AuthUseCases>,
    pub users: Arc<UserUseCases>,
    pub password_reset: Arc<PasswordResetUseCases>,
    pub catalog: Arc<CatalogUseCases>,
    pub interests: Arc<InterestUseCases>,
    pub members: Arc<MemberUseCases>,
    pub emails: Arc<EmailUseCases>,
    pub statistics: Arc<StatisticsUseCases>,
    pub form_ocr: Arc<FormOcrUseCases>,
}

macro_rules! from_ref {
    ($($field:ident: $ty:ty),* $(,)?) => {$(
        impl FromRef<AppState> for Arc<$ty> {
            fn from_ref(state: &AppState) -> Self {
                state.$field.clone()
            }
        }
    )*};
}

from_ref! {
    cookies: CookieSettings,
    login_rate_limiter: RateLimiter,
    auth: AuthUseCases,
    users: UserUseCases,
    password_reset: PasswordResetUseCases,
    catalog: CatalogUseCases,
    interests: InterestUseCases,
    members: MemberUseCases,
    emails: EmailUseCases,
    statistics: StatisticsUseCases,
    form_ocr: FormOcrUseCases,
}
