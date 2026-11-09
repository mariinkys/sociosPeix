use bytes::Bytes;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailCategory {
    /// Newsletters, birthday greetings, sends to members.
    Campaign,
    /// Password resets and other one-off system emails.
    Transactional,
}

impl EmailCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Campaign => "CAMPAIGN",
            Self::Transactional => "TRANSACTIONAL",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "TRANSACTIONAL" => Self::Transactional,
            _ => Self::Campaign,
        }
    }
}

/// An email that has been sent (the stored log entry).
#[derive(Debug, Clone)]
pub struct Email {
    pub id: Uuid,
    pub subject: String,
    /// The body as it should be previewed in the app (may differ from what was
    /// sent when inline images were referenced by `cid:`).
    pub body: String,
    pub provider: String,
    pub recipients: Vec<String>,
    pub category: EmailCategory,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NewEmail {
    pub subject: String,
    pub body: String,
    pub provider: String,
    pub recipients: Vec<String>,
    pub category: EmailCategory,
}

/// A file attached to an outgoing email. `content` is reference-counted so the
/// same upload can be sent in several batches without being copied.
#[derive(Debug, Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: Option<String>,
    pub content: Bytes,
    /// When set, the attachment is inline and referenced from the HTML as `cid:<content_id>`.
    pub content_id: Option<String>,
}

/// A message handed to an email provider.
#[derive(Debug)]
pub struct OutgoingEmail<'a> {
    pub subject: &'a str,
    pub html: &'a str,
    pub recipients: &'a [String],
    /// When true, recipients must not see each other (they go in BCC).
    pub hide_recipients: bool,
    pub attachments: &'a [EmailAttachment],
}

#[derive(Debug, Clone)]
pub struct EmailProviderInfo {
    pub name: String,
    pub daily_limit: u32,
}

#[derive(Debug, Clone)]
pub struct EmailProviderStatus {
    pub provider: String,
    pub daily_limit: i64,
    pub sent_today: i64,
    pub remaining: i64,
}

#[derive(Debug, Clone)]
pub struct MultiEmailCheck {
    pub status: EmailProviderStatus,
    pub total_recipients: i64,
    pub exceeds_limit: bool,
}
