use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, patch},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::{PageResponse, UserResponse},
        extractors::{AdminUser, AuthUser, P, Q, ValidJson},
        validation::{Validate, Validator},
    },
    app_error::{AppError, AppResult},
    domain::pagination::{PageRequest, SortDirection},
    entities::user::Role,
    use_cases::user::{UserSortField, UserUseCases},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/me", get(me))
        .route("/{id}", get(get_by_id).put(update).delete(delete))
        .route("/{id}/role", patch(update_role))
        .route("/{id}/password", patch(change_password))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListParams {
    #[serde(default)]
    page: u32,
    #[serde(default = "default_size")]
    size: u32,
    sort_by: Option<String>,
    sort_dir: Option<String>,
    #[serde(default)]
    search: String,
}

fn default_size() -> u32 {
    10
}

async fn list(
    _admin: AdminUser,
    State(users): State<Arc<UserUseCases>>,
    Q(p): Q<ListParams>,
) -> AppResult<Json<PageResponse<UserResponse>>> {
    let field = match p.sort_by.as_deref() {
        None => UserSortField::CreatedAt,
        Some(s) => UserSortField::parse(s)
            .ok_or_else(|| AppError::Validation(format!("Invalid sort field: {s}")))?,
    };
    let dir = SortDirection::parse(p.sort_dir.as_deref().unwrap_or("desc"));
    let page = users
        .list_users(&p.search, (field, dir), PageRequest::new(p.page, p.size))
        .await?;
    Ok(Json(PageResponse::from_page(page, UserResponse::from)))
}

async fn me(AuthUser(user): AuthUser) -> Json<UserResponse> {
    Json(user.into())
}

async fn get_by_id(
    auth: AuthUser,
    State(users): State<Arc<UserUseCases>>,
    P(id): P<Uuid>,
) -> AppResult<Json<UserResponse>> {
    Ok(Json(users.get_user(id, auth.requester()).await?.into()))
}

#[derive(Deserialize)]
struct UpdateRequest {
    name: Option<String>,
    email: Option<String>,
}

impl Validate for UpdateRequest {
    fn validate(&self) -> Result<(), String> {
        let name = self.name.as_deref().map(str::trim);
        Validator::default()
            .required(name, "Name is required")
            .min_chars(name, 2, "Name must be between 2 and 100 characters")
            .max_chars(name, 100, "Name must be between 2 and 100 characters")
            .required(self.email.as_deref(), "Email is required")
            .email(self.email.as_deref(), "Must be a valid email")
            .finish()
    }
}

async fn update(
    auth: AuthUser,
    State(users): State<Arc<UserUseCases>>,
    P(id): P<Uuid>,
    ValidJson(req): ValidJson<UpdateRequest>,
) -> AppResult<Json<UserResponse>> {
    let user = users
        .update_user(
            id,
            req.name.as_deref().unwrap_or_default().trim(),
            req.email.as_deref().unwrap_or_default().trim(),
            auth.requester(),
        )
        .await?;
    Ok(Json(user.into()))
}

#[derive(Deserialize)]
struct UpdateRoleRequest {
    role: Option<String>,
}

impl Validate for UpdateRoleRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.role.as_deref(), "Role is required")
            .check(
                self.role
                    .as_deref()
                    .is_none_or(|r| Role::parse(r).is_some()),
                "Role must be USER or ADMIN",
            )
            .finish()
    }
}

async fn update_role(
    _admin: AdminUser,
    State(users): State<Arc<UserUseCases>>,
    P(id): P<Uuid>,
    ValidJson(req): ValidJson<UpdateRoleRequest>,
) -> AppResult<Json<UserResponse>> {
    let role = req
        .role
        .as_deref()
        .and_then(Role::parse)
        .unwrap_or(Role::User);
    Ok(Json(users.update_role(id, role).await?.into()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangePasswordRequest {
    /// Required for regular users, ignored for admins.
    current_password: Option<String>,
    new_password: Option<String>,
}

impl Validate for ChangePasswordRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.new_password.as_deref(), "New password is required")
            .min_chars(
                self.new_password.as_deref(),
                8,
                "Password must be at least 8 characters",
            )
            .finish()
    }
}

async fn change_password(
    auth: AuthUser,
    State(users): State<Arc<UserUseCases>>,
    P(id): P<Uuid>,
    ValidJson(req): ValidJson<ChangePasswordRequest>,
) -> AppResult<StatusCode> {
    users
        .change_password(
            id,
            req.current_password.as_deref(),
            req.new_password.as_deref().unwrap_or_default(),
            auth.requester(),
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete(
    auth: AuthUser,
    State(users): State<Arc<UserUseCases>>,
    P(id): P<Uuid>,
) -> AppResult<StatusCode> {
    users.delete_user(id, auth.requester()).await?;
    Ok(StatusCode::NO_CONTENT)
}
