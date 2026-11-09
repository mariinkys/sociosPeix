use async_trait::async_trait;
use serde::Serialize;

use crate::{
    adapters::email::{Sender, check_response, encode_attachment, transport_error},
    app_error::AppResult,
    entities::email::OutgoingEmail,
    use_cases::email::EmailSender,
};

pub const DEFAULT_ENDPOINT: &str = "https://send.api.mailtrap.io/api/send";

pub struct MailtrapSender {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    from: Sender,
    daily_limit: u32,
}

impl MailtrapSender {
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
struct Address<'a> {
    email: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}

impl<'a> Address<'a> {
    fn plain(email: &'a str) -> Self {
        Self { email, name: None }
    }
}

#[derive(Serialize)]
struct Body<'a> {
    from: Address<'a>,
    to: Vec<Address<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bcc: Vec<Address<'a>>,
    subject: &'a str,
    html: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment<'a>>,
}

#[derive(Serialize)]
struct Attachment<'a> {
    filename: &'a str,
    content: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    content_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disposition: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_id: Option<&'a str>,
}

#[async_trait]
impl EmailSender for MailtrapSender {
    fn name(&self) -> &str {
        "MAILTRAP"
    }

    fn daily_limit(&self) -> u32 {
        self.daily_limit
    }

    async fn send(&self, email: &OutgoingEmail<'_>) -> AppResult<()> {
        let recipients: Vec<Address> = email.recipients.iter().map(|r| Address::plain(r)).collect();
        let (to, bcc) = if email.hide_recipients {
            (vec![Address::plain(&self.from.email)], recipients)
        } else {
            (recipients, Vec::new())
        };
        let body = Body {
            from: Address {
                email: &self.from.email,
                name: self.from.name.as_deref(),
            },
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
                    disposition: a.content_id.is_some().then_some("inline"),
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
            .map_err(|e| transport_error("Mailtrap", e))?;
        check_response("Mailtrap", res).await
    }
}
