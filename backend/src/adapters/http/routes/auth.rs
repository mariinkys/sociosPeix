use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::post,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        cookies::{CookieSettings, REFRESH_TOKEN},
        dto::UserResponse,
        extractors::{AdminUser, ValidJson},
        rate_limit::limit_logins,
        validation::{Validate, Validator},
    },
    app_error::AppResult,
    entities::user::User,
    use_cases::{
        auth::{AuthUseCases, SessionTokens},
        password_reset::PasswordResetUseCases,
        user::UserUseCases,
    },
};

pub fn router(state: &AppState) -> Router<AppState> {
    let rate_limited = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route_layer(middleware::from_fn_with_state(
            state.login_rate_limiter.clone(),
            limit_logins,
        ));

    Router::new()
        .merge(rate_limited)
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}

#[derive(Serialize)]
struct AuthResponse {
    id: Uuid,
    email: String,
    role: &'static str,
}

impl From<User> for AuthResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            role: u.role.as_str(),
            email: u.email,
        }
    }
}

fn with_session(jar: CookieJar, cookies: &CookieSettings, tokens: SessionTokens) -> CookieJar {
    jar.add(cookies.access(tokens.access_token))
        .add(cookies.refresh(tokens.refresh_token))
}

#[derive(Deserialize)]
struct LoginRequest {
    email: Option<String>,
    password: Option<String>,
}

impl Validate for LoginRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.email.as_deref(), "Email is required")
            .email(self.email.as_deref(), "Must be a valid email")
            .required(self.password.as_deref(), "Password is required")
            .finish()
    }
}

async fn login(
    State(auth): State<Arc<AuthUseCases>>,
    State(cookies): State<Arc<CookieSettings>>,
    jar: CookieJar,
    ValidJson(req): ValidJson<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let (user, tokens) = auth
        .login(
            req.email.as_deref().unwrap_or_default().trim(),
            req.password.as_deref().unwrap_or_default(),
        )
        .await?;
    Ok((
        with_session(jar, &cookies, tokens),
        Json(AuthResponse::from(user)),
    ))
}

#[derive(Deserialize)]
struct RegisterRequest {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl Validate for RegisterRequest {
    fn validate(&self) -> Result<(), String> {
        let name = self.name.as_deref().map(str::trim);
        Validator::default()
            .required(name, "Name is required")
            .min_chars(name, 2, "Name must be between 2 and 100 characters")
            .max_chars(name, 100, "Name must be between 2 and 100 characters")
            .required(self.email.as_deref(), "Email is required")
            .email(self.email.as_deref(), "Must be a valid email")
            .required(self.password.as_deref(), "Password is required")
            .min_chars(
                self.password.as_deref(),
                8,
                "Password must be at least 8 characters",
            )
            .finish()
    }
}

/// Admins create accounts; there is no public sign-up.
async fn register(
    _admin: AdminUser,
    State(users): State<Arc<UserUseCases>>,
    ValidJson(req): ValidJson<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    let user = users
        .create_user(
            req.name.as_deref().unwrap_or_default().trim(),
            req.email.as_deref().unwrap_or_default().trim(),
            req.password.as_deref().unwrap_or_default(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

async fn refresh(
    State(auth): State<Arc<AuthUseCases>>,
    State(cookies): State<Arc<CookieSettings>>,
    jar: CookieJar,
) -> AppResult<Response> {
    let Some(token) = jar.get(REFRESH_TOKEN).map(|c| c.value().to_owned()) else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };
    let (user, tokens) = auth.refresh(&token).await?;
    Ok((
        with_session(jar, &cookies, tokens),
        Json(AuthResponse::from(user)),
    )
        .into_response())
}

/// Always clears the cookies; also revokes every refresh token of the user
/// when the session is still valid.
async fn logout(
    State(auth): State<Arc<AuthUseCases>>,
    State(cookies): State<Arc<CookieSettings>>,
    jar: CookieJar,
) -> AppResult<impl IntoResponse> {
    if let Some(token) = jar.get(crate::adapters::http::cookies::ACCESS_TOKEN)
        && let Ok(user) = auth.authenticate(token.value()).await
    {
        auth.logout(&user.email).await?;
    }
    let jar = jar.add(cookies.clear_access()).add(cookies.clear_refresh());
    Ok((StatusCode::NO_CONTENT, jar))
}

#[derive(Deserialize)]
struct ForgotPasswordRequest {
    email: Option<String>,
}

impl Validate for ForgotPasswordRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.email.as_deref(), "Email is required")
            .email(self.email.as_deref(), "Must be a valid email")
            .finish()
    }
}

async fn forgot_password(
    State(reset): State<Arc<PasswordResetUseCases>>,
    ValidJson(req): ValidJson<ForgotPasswordRequest>,
) -> AppResult<StatusCode> {
    reset
        .request_reset(req.email.as_deref().unwrap_or_default().trim())
        .await?;
    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResetPasswordRequest {
    email: Option<String>,
    code: Option<String>,
    new_password: Option<String>,
}

impl Validate for ResetPasswordRequest {
    fn validate(&self) -> Result<(), String> {
        let code = self.code.as_deref().unwrap_or_default();
        Validator::default()
            .required(self.email.as_deref(), "Email is required")
            .check(
                code.len() == 6 && code.bytes().all(|b| b.is_ascii_digit()),
                "Code must be 6 digits",
            )
            .required(self.new_password.as_deref(), "New password is required")
            .min_chars(
                self.new_password.as_deref(),
                8,
                "Password must be at least 8 characters",
            )
            .finish()
    }
}

async fn reset_password(
    State(reset): State<Arc<PasswordResetUseCases>>,
    ValidJson(req): ValidJson<ResetPasswordRequest>,
) -> AppResult<StatusCode> {
    reset
        .reset_password(
            req.email.as_deref().unwrap_or_default().trim(),
            req.code.as_deref().unwrap_or_default(),
            req.new_password.as_deref().unwrap_or_default(),
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
