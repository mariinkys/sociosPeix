use std::sync::Arc;

use axum::{Json, Router, extract::State, http::header, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::{
    adapters::http::{app_state::AppState, extractors::AdminUser, routes::members::XLSX},
    app_error::AppResult,
    use_cases::statistics::StatisticsUseCases,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/interests/popularity", get(interest_popularity))
        .route(
            "/interests/popularity/export",
            get(export_interest_popularity),
        )
        .route("/members/email-preference", get(email_preference_stats))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InterestPopularityResponse {
    interest_name: String,
    member_count: i64,
}

async fn interest_popularity(
    _admin: AdminUser,
    State(stats): State<Arc<StatisticsUseCases>>,
) -> AppResult<Json<Vec<InterestPopularityResponse>>> {
    let rows = stats.interest_popularity().await?;
    Ok(Json(
        rows.into_iter()
            .map(|r| InterestPopularityResponse {
                interest_name: r.interest_name,
                member_count: r.member_count,
            })
            .collect(),
    ))
}

async fn export_interest_popularity(
    _admin: AdminUser,
    State(stats): State<Arc<StatisticsUseCases>>,
) -> AppResult<impl IntoResponse> {
    let file = stats.export_interest_popularity().await?;
    Ok((
        [
            (header::CONTENT_TYPE, XLSX),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"interest-popularity.xlsx\"",
            ),
        ],
        file,
    ))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmailPreferenceStatsResponse {
    enabled: i64,
    disabled: i64,
    total: i64,
}

async fn email_preference_stats(
    _admin: AdminUser,
    State(stats): State<Arc<StatisticsUseCases>>,
) -> AppResult<Json<EmailPreferenceStatsResponse>> {
    let row = stats.email_preference_stats().await?;
    Ok(Json(EmailPreferenceStatsResponse {
        enabled: row.enabled,
        disabled: row.disabled,
        total: row.total(),
    }))
}
