use async_trait::async_trait;
use serde::Serialize;

use crate::{
    adapters::email::{Sender, check_response, encode_attachment, transport_error},
    app_error::AppResult,
    entities::email::OutgoingEmail,
    use_cases::email::EmailSender,
};

pub const DEFAULT_ENDPOINT: &str = "https://api.resend.com/emails";

pub struct ResendSender {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    from: Sender,
    daily_limit: u32,
}

impl ResendSender {
    pub fn new(
        client: reqwest::Client,
        endpoint: String,
        api_key: String,
        from: Sender,
        daily_limit: u32,
    ) -> Self {
        Self {
            client,
            endpoint,
            api_key,
            from,
            daily_limit,
        }
    }
}

#[derive(Serialize)]
struct Body<'a> {
    from: &'a str,
    to: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bcc: Vec<&'a str>,
    subject: &'a str,
    html: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment<'a>>,
}

#[derive(Serialize)]
struct Attachment<'a> {
    filename: &'a str,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_id: Option<&'a str>,
}

#[async_trait]
impl EmailSender for ResendSender {
    fn name(&self) -> &str {
        "RESEND"
    }

    fn daily_limit(&self) -> u32 {
        self.daily_limit
    }

    async fn send(&self, email: &OutgoingEmail<'_>) -> AppResult<()> {
        let recipients: Vec<&str> = email.recipients.iter().map(String::as_str).collect();
        let (to, bcc) = if email.hide_recipients {
            (vec![self.from.email.as_str()], recipients)
        } else {
            (recipients, Vec::new())
        };
        let body = Body {
            from: &self.from.raw,
            to,
            bcc,
            subject: email.subject,
            html: email.html,
            attachments: email
                .attachments
                .iter()
                .map(|a| Attachment {
                    filename: &a.filename,
                    content: encode_attachment(a),
                    content_type: a.content_type.as_deref(),
                    content_id: a.content_id.as_deref(),
                })
                .collect(),
        };
        let res = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| transport_error("Resend", e))?;
        check_response("Resend", res).await
    }
}
