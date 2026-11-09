//! Email delivery providers. Each one talks to the provider's HTTP API through
//! a shared `reqwest::Client` (one connection pool for the whole process).

use std::time::Duration;

use base64::{Engine, engine::general_purpose::STANDARD};

use crate::{
    app_error::{AppError, AppResult},
    entities::email::EmailAttachment,
};

pub mod mailtrap;
pub mod resend;

pub fn http_client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .user_agent(concat!("sociospeix/", env!("CARGO_PKG_VERSION")))
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(60))
        .pool_idle_timeout(Duration::from_secs(90))
        .build()?)
}

/// A parsed `From` address: either `email@x.com` or `Display Name <email@x.com>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    pub name: Option<String>,
    pub email: String,
    /// The original string, as Resend accepts it verbatim.
    pub raw: String,
}

impl Sender {
    pub fn parse(raw: &str) -> Self {
        let raw = raw.trim().to_owned();
        if let (Some(start), true) = (raw.rfind('<'), raw.ends_with('>')) {
            let email = raw[start + 1..raw.len() - 1].trim().to_owned();
            let name = raw[..start].trim().trim_matches('"').trim();
            return Self {
                name: (!name.is_empty()).then(|| name.to_owned()),
                email,
                raw,
            };
        }
        Self {
            name: None,
            email: raw.clone(),
            raw,
        }
    }
}

pub(crate) fn encode_attachment(a: &EmailAttachment) -> String {
    STANDARD.encode(&a.content)
}

/// Turns a non-2xx provider response into an error with a bounded message.
pub(crate) async fn check_response(provider: &str, res: reqwest::Response) -> AppResult<()> {
    let status = res.status();
    if status.is_success() {
        return Ok(());
    }
    let body = res.text().await.unwrap_or_default();
    let body: String = body.chars().take(500).collect();
    Err(AppError::EmailSend(format!(
        "{provider} error ({status}): {body}"
    )))
}

pub(crate) fn transport_error(provider: &str, e: reqwest::Error) -> AppError {
    AppError::EmailSend(format!("{provider} error: {e}"))
}

#[cfg(test)]
mod tests {
    use super::Sender;

    #[test]
    fn parses_from_addresses() {
        let s = Sender::parse("Hotel Casa Peix <noreply@peix.es>");
        assert_eq!(s.name.as_deref(), Some("Hotel Casa Peix"));
        assert_eq!(s.email, "noreply@peix.es");
        let s = Sender::parse("noreply@peix.es");
        assert_eq!((s.name, s.email.as_str()), (None, "noreply@peix.es"));
    }
}
