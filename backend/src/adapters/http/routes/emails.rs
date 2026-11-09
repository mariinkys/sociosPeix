use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, RawQuery, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        dto::{PageResponse, serialize_utc},
        extractors::{AdminUser, AuthUser, P, Q, ValidJson, int_list},
        validation::{Validate, Validator},
    },
    app_error::{AppError, AppResult},
    domain::pagination::PageRequest,
    entities::email::{Email, EmailAttachment, EmailProviderStatus},
    use_cases::email::{EmailContent, EmailUseCases},
};

pub fn router(upload_limit: usize) -> Router<AppState> {
    let sends = Router::new()
        .route("/send/member/{member_id}", post(send_to_member))
        .route("/send/all", post(send_to_all))
        .route("/send/interests", post(send_to_interests))
        .layer(DefaultBodyLimit::max(upload_limit));

    Router::new()
        .merge(sends)
        .route("/", get(list))
        .route("/today", get(today))
        .route("/member/{member_id}", get(by_member))
        .route("/provider/multi-check", get(multi_check))
        .route("/provider/status", get(provider_status))
        .route("/providers", get(list_providers))
        .route("/active-provider", put(set_active_provider))
        .route("/{id}", get(get_by_id))
}

// ---------- responses ----------

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EmailResponse {
    id: Uuid,
    subject: String,
    provider: String,
    body: String,
    recipient_count: usize,
    recipient_emails: Vec<String>,
    category: &'static str,
    #[serde(serialize_with = "serialize_utc")]
    created_at: NaiveDateTime,
}

impl From<Email> for EmailResponse {
    fn from(e: Email) -> Self {
        Self {
            id: e.id,
            subject: e.subject,
            provider: e.provider,
            body: e.body,
            recipient_count: e.recipients.len(),
            recipient_emails: e.recipients,
            category: e.category.as_str(),
            created_at: e.created_at,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderStatusResponse {
    provider: String,
    daily_limit: i64,
    sent_today: i64,
    remaining: i64,
}

impl From<EmailProviderStatus> for ProviderStatusResponse {
    fn from(s: EmailProviderStatus) -> Self {
        Self {
            provider: s.provider,
            daily_limit: s.daily_limit,
            sent_today: s.sent_today,
            remaining: s.remaining,
        }
    }
}

// ---------- reads ----------

#[derive(Deserialize)]
struct PageParams {
    #[serde(default)]
    page: u32,
    #[serde(default = "default_size")]
    size: u32,
}

fn default_size() -> u32 {
    10
}

async fn list(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    Q(p): Q<PageParams>,
) -> AppResult<Json<PageResponse<EmailResponse>>> {
    let page = emails.list(PageRequest::new(p.page, p.size)).await?;
    Ok(Json(PageResponse::from_page(page, EmailResponse::from)))
}

async fn today(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
) -> AppResult<Json<Vec<EmailResponse>>> {
    Ok(Json(
        emails
            .sent_today()
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    ))
}

async fn get_by_id(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    P(id): P<Uuid>,
) -> AppResult<Json<EmailResponse>> {
    Ok(Json(emails.get(id).await?.into()))
}

async fn by_member(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    P(member_id): P<Uuid>,
    Q(p): Q<PageParams>,
) -> AppResult<Json<PageResponse<EmailResponse>>> {
    let page = emails
        .sent_to_member(member_id, PageRequest::new(p.page, p.size))
        .await?;
    Ok(Json(PageResponse::from_page(page, EmailResponse::from)))
}

// ---------- provider ----------

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MultiCheckResponse {
    #[serde(flatten)]
    status: ProviderStatusResponse,
    total_recipients: i64,
    exceeds_limit: bool,
}

/// Would a send to these interests (or to everyone) fit in today's quota?
async fn multi_check(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    RawQuery(raw): RawQuery,
) -> AppResult<Json<MultiCheckResponse>> {
    let ids = int_list(raw.as_deref(), "interestIds")?;
    let check = emails.check_multi_send(Some(&ids)).await?;
    Ok(Json(MultiCheckResponse {
        status: check.status.into(),
        total_recipients: check.total_recipients,
        exceeds_limit: check.exceeds_limit,
    }))
}

async fn provider_status(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
) -> AppResult<Json<ProviderStatusResponse>> {
    Ok(Json(emails.provider_status().await?.into()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderInfoResponse {
    name: String,
    daily_limit: u32,
}

async fn list_providers(
    _admin: AdminUser,
    State(emails): State<Arc<EmailUseCases>>,
) -> Json<Vec<ProviderInfoResponse>> {
    Json(
        emails
            .list_providers()
            .into_iter()
            .map(|p| ProviderInfoResponse {
                name: p.name,
                daily_limit: p.daily_limit,
            })
            .collect(),
    )
}

#[derive(Deserialize)]
struct SetActiveProviderRequest {
    provider: Option<String>,
}

impl Validate for SetActiveProviderRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.provider.as_deref(), "Provider is required")
            .finish()
    }
}

async fn set_active_provider(
    _admin: AdminUser,
    State(emails): State<Arc<EmailUseCases>>,
    ValidJson(req): ValidJson<SetActiveProviderRequest>,
) -> AppResult<StatusCode> {
    emails
        .set_active_provider(req.provider.as_deref().unwrap_or_default().trim())
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------- sends (multipart) ----------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendEmailRequest {
    subject: Option<String>,
    html_body: Option<String>,
    preview_html_body: Option<String>,
}

impl Validate for SendEmailRequest {
    fn validate(&self) -> Result<(), String> {
        Validator::default()
            .required(self.subject.as_deref(), "Subject is required")
            .required(self.html_body.as_deref(), "Body is required")
            .finish()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SendToInterestsRequest {
    #[serde(flatten)]
    email: SendEmailRequest,
    interest_ids: Option<Vec<i32>>,
}

impl Validate for SendToInterestsRequest {
    fn validate(&self) -> Result<(), String> {
        let mut v = Validator::default();
        if let Err(e) = self.email.validate() {
            v.check(false, &e);
        }
        v.check(
            self.interest_ids
                .as_ref()
                .is_some_and(|ids| !ids.is_empty()),
            "At least one interest must be specified",
        )
        .finish()
    }
}

/// The parts of a send request: the JSON `data` part plus uploaded files
struct SendForm<T> {
    data: T,
    attachments: Vec<EmailAttachment>,
}

impl<T> SendForm<T> {
    fn content(self, request: impl FnOnce(&T) -> &SendEmailRequest) -> (T, EmailContent) {
        let req = request(&self.data);
        let subject = req.subject.as_deref().unwrap_or_default().trim().to_owned();
        let html = req.html_body.clone().unwrap_or_default();
        let preview = req
            .preview_html_body
            .clone()
            .unwrap_or_else(|| html.clone());
        let content = EmailContent {
            subject,
            html_body: html,
            preview_body: preview,
            attachments: self.attachments,
        };
        (self.data, content)
    }
}

fn bad_multipart(e: axum::extract::multipart::MultipartError) -> AppError {
    AppError::Validation(e.body_text())
}

async fn read_send_form<T: DeserializeOwned + Validate>(
    mut multipart: Multipart,
) -> AppResult<SendForm<T>> {
    let mut data = None;
    let mut attachments = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(bad_multipart)? {
        let name = field.name().unwrap_or_default().to_owned();
        match name.as_str() {
            "data" => {
                let bytes = field.bytes().await.map_err(bad_multipart)?;
                let parsed: T = serde_json::from_slice(&bytes)
                    .map_err(|e| AppError::Validation(format!("Invalid 'data' part: {e}")))?;
                parsed.validate().map_err(AppError::Validation)?;
                data = Some(parsed);
            }
            "attachments" | "inlineImages" => {
                let filename = field.file_name().unwrap_or("attachment").to_owned();
                let content_type = field.content_type().map(str::to_owned);
                let content = field.bytes().await.map_err(bad_multipart)?;
                // The frontend names each inline image after the content-ID
                // it referenced as `cid:` in the HTML body.
                let content_id = (name == "inlineImages").then(|| filename.clone());
                attachments.push(EmailAttachment {
                    filename,
                    content_type,
                    content,
                    content_id,
                });
            }
            _ => {} // ignore unknown parts
        }
    }

    let data = data.ok_or_else(|| AppError::Validation("Missing 'data' part".into()))?;
    Ok(SendForm { data, attachments })
}

async fn send_to_member(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    P(member_id): P<Uuid>,
    multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    let form = read_send_form::<SendEmailRequest>(multipart).await?;
    let (_, content) = form.content(|r| r);
    let email = emails.send_to_member(member_id, content).await?;
    Ok((StatusCode::CREATED, Json(EmailResponse::from(email))))
}

async fn send_to_all(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    let form = read_send_form::<SendEmailRequest>(multipart).await?;
    let (_, content) = form.content(|r| r);
    let email = emails.send_to_all(content).await?;
    Ok((StatusCode::CREATED, Json(EmailResponse::from(email))))
}

/// Sends to members having *any* of the given interests.
async fn send_to_interests(
    _user: AuthUser,
    State(emails): State<Arc<EmailUseCases>>,
    multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    let form = read_send_form::<SendToInterestsRequest>(multipart).await?;
    let (data, content) = form.content(|r| &r.email);
    let ids = data.interest_ids.unwrap_or_default();
    let email = emails.send_to_interests(&ids, content).await?;
    Ok((StatusCode::CREATED, Json(EmailResponse::from(email))))
}
