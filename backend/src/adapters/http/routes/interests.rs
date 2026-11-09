use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Deserialize;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::InterestResponse,
        extractors::{AuthUser, P, ValidJson},
        validation::{Validate, Validator},
    },
    app_error::AppResult,
    use_cases::interest::InterestUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_by_id).put(update).delete(delete))
}

#[derive(Deserialize)]
struct InterestRequest {
    name: Option<String>,
    description: Option<String>,
}

impl Validate for InterestRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.name.as_deref(), "Name is required")
            .max_chars(
                self.name.as_deref().map(str::trim),
                100,
                "Name must not exceed 100 characters",
            )
            .finish()
    }
}

impl InterestRequest {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or_default().trim()
    }

    fn description(&self) -> &str {
        self.description.as_deref().unwrap_or_default()
    }
}

async fn list(
    _user: AuthUser,
    State(interests): State<Arc<InterestUseCases>>,
) -> AppResult<Json<Vec<InterestResponse>>> {
    Ok(Json(
        interests.list().await?.iter().map(Into::into).collect(),
    ))
}

async fn get_by_id(
    _user: AuthUser,
    State(interests): State<Arc<InterestUseCases>>,
    P(id): P<i32>,
) -> AppResult<Json<InterestResponse>> {
    Ok(Json(interests.get(id).await?.into()))
}

async fn create(
    _user: AuthUser,
    State(interests): State<Arc<InterestUseCases>>,
    ValidJson(req): ValidJson<InterestRequest>,
) -> AppResult<impl IntoResponse> {
    let created = interests.create(req.name(), req.description()).await?;
    Ok((StatusCode::CREATED, Json(InterestResponse::from(created))))
}

async fn update(
    _user: AuthUser,
    State(interests): State<Arc<InterestUseCases>>,
    P(id): P<i32>,
    ValidJson(req): ValidJson<InterestRequest>,
) -> AppResult<Json<InterestResponse>> {
    Ok(Json(
        interests
            .update(id, req.name(), req.description())
            .await?
            .into(),
    ))
}

async fn delete(
    _user: AuthUser,
    State(interests): State<Arc<InterestUseCases>>,
    P(id): P<i32>,
) -> AppResult<StatusCode> {
    interests.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
