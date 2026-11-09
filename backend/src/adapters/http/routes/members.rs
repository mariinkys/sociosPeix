use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{RawQuery, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::{InterestResponse, PageResponse, serialize_utc},
        extractors::{AuthUser, P, Q, ValidJson, int_list},
        validation::{Validate, Validator},
    },
    app_error::{AppError, AppResult},
    domain::pagination::{PageRequest, SortDirection},
    entities::member::{Member, MemberData, MemberFilter, MemberSortField},
    use_cases::member::MemberUseCases,
};

pub const XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/export", get(export))
        .route("/birthdays/today", get(todays_birthdays))
        .route("/{id}", get(get_by_id).put(update).delete(delete))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MemberResponse {
    id: Uuid,
    full_name: String,
    name: String,
    surname: String,
    second_surname: String,
    email: String,
    birthdate: Option<NaiveDate>,
    phone: String,
    notes: String,
    emails_disabled: bool,
    gender_id: Option<i32>,
    country_id: Option<i32>,
    #[serde(serialize_with = "serialize_utc")]
    created_at: NaiveDateTime,
    interests: Vec<InterestResponse>,
}

impl From<Member> for MemberResponse {
    fn from(m: Member) -> Self {
        Self {
            id: m.id,
            full_name: m.full_name(),
            name: m.name,
            surname: m.surname,
            second_surname: m.second_surname,
            email: m.email,
            birthdate: m.birthdate,
            phone: m.phone,
            notes: m.notes,
            emails_disabled: m.emails_disabled,
            gender_id: m.gender_id,
            country_id: m.country_id,
            created_at: m.created_at,
            interests: m.interests.into_iter().map(Into::into).collect(),
        }
    }
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

#[derive(Deserialize)]
struct SearchParams {
    #[serde(default)]
    search: String,
}

async fn list(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
    Q(p): Q<ListParams>,
    RawQuery(raw): RawQuery,
) -> AppResult<Json<PageResponse<MemberResponse>>> {
    let field = match p.sort_by.as_deref() {
        None => MemberSortField::default(),
        Some(s) => MemberSortField::parse(s)
            .ok_or_else(|| AppError::Validation(format!("Invalid sort field: {s}")))?,
    };
    let dir = SortDirection::parse(p.sort_dir.as_deref().unwrap_or("asc"));
    let filter = MemberFilter::new(p.search, int_list(raw.as_deref(), "interestIds")?);
    let page = members
        .list(&filter, (field, dir), PageRequest::new(p.page, p.size))
        .await?;
    Ok(Json(PageResponse::from_page(page, MemberResponse::from)))
}

async fn export(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
    Q(p): Q<SearchParams>,
    RawQuery(raw): RawQuery,
) -> AppResult<impl IntoResponse> {
    let filter = MemberFilter::new(p.search, int_list(raw.as_deref(), "interestIds")?);
    let file = members.export(&filter).await?;
    Ok((
        [
            (header::CONTENT_TYPE, XLSX),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"members.xlsx\"",
            ),
        ],
        file,
    ))
}

async fn todays_birthdays(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
) -> AppResult<Json<Vec<MemberResponse>>> {
    let list = members.todays_birthdays().await?;
    Ok(Json(list.into_iter().map(Into::into).collect()))
}

async fn get_by_id(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
    P(id): P<Uuid>,
) -> AppResult<Json<MemberResponse>> {
    Ok(Json(members.get(id).await?.into()))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemberRequest {
    name: Option<String>,
    surname: Option<String>,
    second_surname: Option<String>,
    email: Option<String>,
    birthdate: Option<String>,
    phone: Option<String>,
    notes: Option<String>,
    emails_disabled: Option<bool>,
    gender_id: Option<i32>,
    country_id: Option<i32>,
    interest_ids: Option<Vec<i32>>,
}

fn parse_birthdate(s: Option<&str>) -> Result<Option<NaiveDate>, ()> {
    match s.map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok(None),
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(Some)
            .map_err(|_| ()),
    }
}

impl Validate for MemberRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.name.as_deref(), "Name is required")
            .max_chars(
                self.name.as_deref(),
                100,
                "Name must not exceed 100 characters",
            )
            .required(self.surname.as_deref(), "Surname is required")
            .max_chars(
                self.surname.as_deref(),
                100,
                "Surname must not exceed 100 characters",
            )
            .max_chars(
                self.second_surname.as_deref(),
                100,
                "Second surname must not exceed 100 characters",
            )
            .required(self.email.as_deref(), "Email is required")
            .email(self.email.as_deref(), "Email must be valid")
            .check(
                parse_birthdate(self.birthdate.as_deref()).is_ok(),
                "Invalid date format. Expected yyyy-MM-dd",
            )
            .max_chars(
                self.phone.as_deref(),
                30,
                "Phone must not exceed 30 characters",
            )
            .finish()
    }
}

impl From<MemberRequest> for MemberData {
    fn from(r: MemberRequest) -> Self {
        let text = |v: Option<String>| v.map(|s| s.trim().to_owned()).unwrap_or_default();
        let mut interest_ids = r.interest_ids.unwrap_or_default();
        interest_ids.sort_unstable();
        interest_ids.dedup();
        MemberData {
            birthdate: parse_birthdate(r.birthdate.as_deref()).ok().flatten(),
            name: text(r.name),
            surname: text(r.surname),
            second_surname: text(r.second_surname),
            email: text(r.email),
            phone: text(r.phone),
            // Notes are free text; keep the user's formatting.
            notes: r.notes.unwrap_or_default(),
            emails_disabled: r.emails_disabled.unwrap_or(false),
            gender_id: r.gender_id,
            country_id: r.country_id,
            interest_ids,
        }
    }
}

async fn create(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
    ValidJson(req): ValidJson<MemberRequest>,
) -> AppResult<impl IntoResponse> {
    let member = members.create(req.into()).await?;
    Ok((StatusCode::CREATED, Json(MemberResponse::from(member))))
}

async fn update(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
    P(id): P<Uuid>,
    ValidJson(req): ValidJson<MemberRequest>,
) -> AppResult<Json<MemberResponse>> {
    Ok(Json(members.update(id, req.into()).await?.into()))
}

async fn delete(
    _user: AuthUser,
    State(members): State<Arc<MemberUseCases>>,
    P(id): P<Uuid>,
) -> AppResult<StatusCode> {
    members.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
