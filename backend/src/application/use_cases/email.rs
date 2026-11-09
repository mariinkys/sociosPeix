use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use tokio::sync::Mutex;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    clock::Clock,
    domain::pagination::{Page, PageRequest},
    entities::email::{
        Email, EmailAttachment, EmailCategory, EmailProviderInfo, EmailProviderStatus,
        MultiEmailCheck, NewEmail, OutgoingEmail,
    },
    use_cases::member::MemberPersistence,
};

#[async_trait]
pub trait EmailPersistence: Send + Sync {
    async fn save_email(&self, email: &NewEmail) -> AppResult<Email>;
    async fn find_email(&self, id: Uuid) -> AppResult<Option<Email>>;
    async fn list_emails(&self, page: PageRequest) -> AppResult<Page<Email>>;
    async fn list_emails_since(&self, since: NaiveDateTime) -> AppResult<Vec<Email>>;
    async fn list_emails_for_member(
        &self,
        member_id: Uuid,
        page: PageRequest,
    ) -> AppResult<Page<Email>>;
    /// Number of recipients the provider was asked to deliver to since `since`.
    async fn count_recipients_since(&self, provider: &str, since: NaiveDateTime) -> AppResult<i64>;
    async fn was_sent_since(
        &self,
        recipient: &str,
        subject: &str,
        since: NaiveDateTime,
    ) -> AppResult<bool>;
}

#[async_trait]
pub trait EmailSettingsPersistence: Send + Sync {
    async fn active_provider(&self) -> AppResult<Option<String>>;
    async fn set_active_provider(&self, name: &str) -> AppResult<()>;
}

/// An email delivery service (Resend, Mailtrap, ...). Only providers that are
/// configured (have credentials) are registered.
#[async_trait]
pub trait EmailSender: Send + Sync {
    /// Stored with each sent email, e.g. `RESEND`.
    fn name(&self) -> &str;
    fn daily_limit(&self) -> u32;
    async fn send(&self, email: &OutgoingEmail<'_>) -> AppResult<()>;
}

/// Providers cap recipients per message (Resend: 50 across to/cc/bcc). The
/// sender's own address takes the `to` slot when recipients are hidden.
pub const MAX_RECIPIENTS_PER_MESSAGE: usize = 49;

pub struct EmailUseCases {
    emails: Arc<dyn EmailPersistence>,
    settings: Arc<dyn EmailSettingsPersistence>,
    members: Arc<dyn MemberPersistence>,
    providers: Vec<Arc<dyn EmailSender>>,
    clock: Clock,
    /// Serialises sends so two concurrent requests can't both pass the
    /// daily-limit check and overshoot the quota together.
    send_lock: Mutex<()>,
}

/// Content of an email to send.
pub struct EmailContent {
    pub subject: String,
    /// What is actually sent; may reference inline images with `cid:`.
    pub html_body: String,
    /// What is stored for previews in the app.
    pub preview_body: String,
    pub attachments: Vec<EmailAttachment>,
}

impl EmailUseCases {
    pub fn new(
        emails: Arc<dyn EmailPersistence>,
        settings: Arc<dyn EmailSettingsPersistence>,
        members: Arc<dyn MemberPersistence>,
        providers: Vec<Arc<dyn EmailSender>>,
        clock: Clock,
    ) -> Self {
        if providers.is_empty() {
            warn!("no email provider is configured; sending email will fail");
        }
        Self {
            emails,
            settings,
            members,
            providers,
            clock,
            send_lock: Mutex::new(()),
        }
    }

    #[instrument(skip(self, content), fields(subject = %content.subject))]
    pub async fn send_to_member(&self, member_id: Uuid, content: EmailContent) -> AppResult<Email> {
        let email =
            self.members.member_email(member_id).await?.ok_or_else(|| {
                AppError::NotFound(format!("Member not found with id: {member_id}"))
            })?;
        if email.trim().is_empty() {
            return Err(AppError::EmailSend("Member has no email address".into()));
        }
        self.send(vec![email], content, EmailCategory::Campaign)
            .await
    }

    #[instrument(skip(self, content), fields(subject = %content.subject))]
    pub async fn send_to_all(&self, content: EmailContent) -> AppResult<Email> {
        let recipients = self.members.recipient_emails(None).await?;
        if recipients.is_empty() {
            return Err(AppError::EmailSend(
                "No members with enabled emails and email addresses found".into(),
            ));
        }
        self.send(recipients, content, EmailCategory::Campaign)
            .await
    }

    /// Sends to every member that has *any* of the given interests.
    #[instrument(skip(self, content), fields(subject = %content.subject))]
    pub async fn send_to_interests(
        &self,
        interest_ids: &[i32],
        content: EmailContent,
    ) -> AppResult<Email> {
        if interest_ids.is_empty() {
            return Err(AppError::Validation(
                "At least one interest must be specified".into(),
            ));
        }
        let recipients = self.members.recipient_emails(Some(interest_ids)).await?;
        if recipients.is_empty() {
            return Err(AppError::EmailSend(
                "No members with email addresses found for the given interests".into(),
            ));
        }
        self.send(recipients, content, EmailCategory::Campaign)
            .await
    }

    pub async fn send_transactional(
        &self,
        recipient: &str,
        subject: &str,
        html: &str,
    ) -> AppResult<Email> {
        if recipient.trim().is_empty() {
            return Err(AppError::EmailSend("Recipient email is required".into()));
        }
        let content = EmailContent {
            subject: subject.to_owned(),
            html_body: html.to_owned(),
            preview_body: html.to_owned(),
            attachments: Vec::new(),
        };
        self.send(
            vec![recipient.to_owned()],
            content,
            EmailCategory::Transactional,
        )
        .await
    }

    pub async fn list(&self, page: PageRequest) -> AppResult<Page<Email>> {
        self.emails.list_emails(page).await
    }

    pub async fn sent_today(&self) -> AppResult<Vec<Email>> {
        self.emails
            .list_emails_since(self.clock.start_of_today_utc())
            .await
    }

    pub async fn sent_to_member(
        &self,
        member_id: Uuid,
        page: PageRequest,
    ) -> AppResult<Page<Email>> {
        self.emails.list_emails_for_member(member_id, page).await
    }

    pub async fn get(&self, id: Uuid) -> AppResult<Email> {
        self.emails
            .find_email(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Email not found with id: {id}")))
    }

    pub async fn was_sent_today(&self, recipient: &str, subject: &str) -> AppResult<bool> {
        self.emails
            .was_sent_since(recipient, subject, self.clock.start_of_today_utc())
            .await
    }

    pub async fn provider_status(&self) -> AppResult<EmailProviderStatus> {
        let provider = self.active_provider().await?;
        self.status_of(provider.as_ref()).await
    }

    pub fn list_providers(&self) -> Vec<EmailProviderInfo> {
        self.providers
            .iter()
            .map(|p| EmailProviderInfo {
                name: p.name().to_owned(),
                daily_limit: p.daily_limit(),
            })
            .collect()
    }

    #[instrument(skip(self))]
    pub async fn set_active_provider(&self, name: &str) -> AppResult<()> {
        if !self.providers.iter().any(|p| p.name() == name) {
            return Err(AppError::Validation(format!(
                "Unknown email provider: {name}"
            )));
        }
        self.settings.set_active_provider(name).await?;
        info!(provider = name, "active email provider switched");
        Ok(())
    }

    /// Would sending to these interests (or to everyone, when `None`/empty) fit
    /// in what's left of today's quota?
    pub async fn check_multi_send(
        &self,
        interest_ids: Option<&[i32]>,
    ) -> AppResult<MultiEmailCheck> {
        let interest_ids = interest_ids.filter(|ids| !ids.is_empty());
        let recipients = self.members.recipient_emails(interest_ids).await?;
        let status = self.provider_status().await?;
        let total_recipients = recipients.len() as i64;
        Ok(MultiEmailCheck {
            exceeds_limit: total_recipients > status.remaining,
            total_recipients,
            status,
        })
    }

    async fn status_of(&self, provider: &dyn EmailSender) -> AppResult<EmailProviderStatus> {
        let daily_limit = i64::from(provider.daily_limit());
        let sent_today = self
            .emails
            .count_recipients_since(provider.name(), self.clock.start_of_today_utc())
            .await?;
        Ok(EmailProviderStatus {
            provider: provider.name().to_owned(),
            daily_limit,
            sent_today,
            remaining: daily_limit - sent_today,
        })
    }

    /// The provider chosen in settings. If it isn't configured in this
    /// deployment, fall back to the first one that is.
    async fn active_provider(&self) -> AppResult<Arc<dyn EmailSender>> {
        let wanted = self.settings.active_provider().await?;
        if let Some(name) = &wanted
            && let Some(p) = self.providers.iter().find(|p| p.name() == name)
        {
            return Ok(p.clone());
        }
        let fallback = self
            .providers
            .first()
            .cloned()
            .ok_or_else(|| AppError::EmailSend("No email provider is configured".into()))?;
        warn!(
            wanted = wanted.as_deref().unwrap_or("<none>"),
            using = fallback.name(),
            "active email provider is not configured; falling back"
        );
        Ok(fallback)
    }

    async fn send(
        &self,
        recipients: Vec<String>,
        content: EmailContent,
        category: EmailCategory,
    ) -> AppResult<Email> {
        let _guard = self.send_lock.lock().await;
        let provider = self.active_provider().await?;

        let status = self.status_of(provider.as_ref()).await?;
        let requested = recipients.len() as i64;
        if requested > status.remaining {
            return Err(AppError::TooManyRequests(format!(
                "Daily send limit reached for provider '{}': limit is {}, already sent {} today, \
                 attempted to send to {} more recipients",
                status.provider, status.daily_limit, status.sent_today, requested
            )));
        }
        debug!(
            provider = provider.name(),
            used = status.sent_today,
            limit = status.daily_limit,
            requested,
            "daily limit check passed"
        );

        // Members must never see each other's addresses, so multi-recipient
        // sends go out as BCC batches.
        let hide_recipients = recipients.len() > 1;
        let mut delivered = 0;
        let mut failure = None;
        for batch in recipients.chunks(MAX_RECIPIENTS_PER_MESSAGE) {
            let message = OutgoingEmail {
                subject: &content.subject,
                html: &content.html_body,
                recipients: batch,
                hide_recipients,
                attachments: &content.attachments,
            };
            match provider.send(&message).await {
                Ok(()) => delivered += batch.len(),
                Err(e) => {
                    failure = Some(e);
                    break;
                }
            }
        }

        if delivered == 0 {
            let e = failure.unwrap_or_else(|| AppError::EmailSend("Nothing was sent".into()));
            return Err(match e {
                e @ AppError::EmailSend(_) => e,
                other => AppError::EmailSend(format!("Failed to send email: {other}")),
            });
        }

        // Record what actually went out (so quota accounting stays right even
        // if a later batch failed).
        let mut recipients = recipients;
        recipients.truncate(delivered);
        let saved = self
            .emails
            .save_email(&NewEmail {
                subject: content.subject,
                body: content.preview_body,
                provider: provider.name().to_owned(),
                recipients,
                category,
            })
            .await?;
        info!(
            provider = provider.name(),
            recipients = delivered,
            category = category.as_str(),
            "email sent"
        );

        if let Some(e) = failure {
            return Err(AppError::EmailSend(format!(
                "Email was only sent to {delivered} of {requested} recipients before the provider failed: {e}"
            )));
        }
        Ok(saved)
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use std::sync::Mutex as StdMutex;

    #[derive(Default)]
    pub struct RecordingSender {
        pub sent: StdMutex<Vec<(Vec<String>, bool)>>,
        pub fail_after: Option<usize>,
    }

    #[async_trait]
    impl EmailSender for RecordingSender {
        fn name(&self) -> &str {
            "FAKE"
        }
        fn daily_limit(&self) -> u32 {
            100
        }
        async fn send(&self, email: &OutgoingEmail<'_>) -> AppResult<()> {
            let mut sent = self.sent.lock().unwrap();
            if self.fail_after.is_some_and(|n| sent.len() >= n) {
                return Err(AppError::EmailSend("boom".into()));
            }
            sent.push((email.recipients.to_vec(), email.hide_recipients));
            Ok(())
        }
    }

    #[derive(Default)]
    pub struct InMemoryEmails {
        pub saved: StdMutex<Vec<Email>>,
    }

    #[async_trait]
    impl EmailPersistence for InMemoryEmails {
        async fn save_email(&self, e: &NewEmail) -> AppResult<Email> {
            let email = Email {
                id: Uuid::new_v4(),
                subject: e.subject.clone(),
                body: e.body.clone(),
                provider: e.provider.clone(),
                recipients: e.recipients.clone(),
                category: e.category,
                created_at: chrono::Utc::now().naive_utc(),
            };
            self.saved.lock().unwrap().push(email.clone());
            Ok(email)
        }
        async fn find_email(&self, _: Uuid) -> AppResult<Option<Email>> {
            Ok(None)
        }
        async fn list_emails(&self, page: PageRequest) -> AppResult<Page<Email>> {
            Ok(Page {
                items: vec![],
                request: page,
                total: 0,
            })
        }
        async fn list_emails_since(&self, _: NaiveDateTime) -> AppResult<Vec<Email>> {
            Ok(vec![])
        }
        async fn list_emails_for_member(
            &self,
            _: Uuid,
            page: PageRequest,
        ) -> AppResult<Page<Email>> {
            Ok(Page {
                items: vec![],
                request: page,
                total: 0,
            })
        }
        async fn count_recipients_since(&self, p: &str, _: NaiveDateTime) -> AppResult<i64> {
            let saved = self.saved.lock().unwrap();
            Ok(saved
                .iter()
                .filter(|e| e.provider == p)
                .map(|e| e.recipients.len() as i64)
                .sum())
        }
        async fn was_sent_since(&self, r: &str, s: &str, _: NaiveDateTime) -> AppResult<bool> {
            let saved = self.saved.lock().unwrap();
            Ok(saved
                .iter()
                .any(|e| e.subject == s && e.recipients.iter().any(|x| x == r)))
        }
    }

    pub struct FixedSettings;

    #[async_trait]
    impl EmailSettingsPersistence for FixedSettings {
        async fn active_provider(&self) -> AppResult<Option<String>> {
            Ok(Some("RESEND".into())) // not configured → falls back to FAKE
        }
        async fn set_active_provider(&self, _: &str) -> AppResult<()> {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use crate::domain::pagination::SortDirection;
    use crate::entities::member::{
        Member, MemberData, MemberExportRow, MemberFilter, MemberSortField,
    };

    struct Recipients(Vec<String>);

    #[async_trait]
    impl MemberPersistence for Recipients {
        async fn find_member(&self, _: Uuid) -> AppResult<Option<Member>> {
            unimplemented!()
        }
        async fn list_members(
            &self,
            _: &MemberFilter,
            _: (MemberSortField, SortDirection),
            _: PageRequest,
        ) -> AppResult<Page<Member>> {
            unimplemented!()
        }
        async fn export_members(&self, _: &MemberFilter) -> AppResult<Vec<MemberExportRow>> {
            unimplemented!()
        }
        async fn members_with_birthday(&self, _: u32, _: u32) -> AppResult<Vec<Member>> {
            unimplemented!()
        }
        async fn create_member(&self, _: &MemberData) -> AppResult<Uuid> {
            unimplemented!()
        }
        async fn update_member(&self, _: Uuid, _: &MemberData) -> AppResult<bool> {
            unimplemented!()
        }
        async fn delete_member(&self, _: Uuid) -> AppResult<bool> {
            unimplemented!()
        }
        async fn member_email(&self, _: Uuid) -> AppResult<Option<String>> {
            Ok(self.0.first().cloned())
        }
        async fn recipient_emails(&self, _: Option<&[i32]>) -> AppResult<Vec<String>> {
            Ok(self.0.clone())
        }
    }

    fn content() -> EmailContent {
        EmailContent {
            subject: "Hi".into(),
            html_body: "<p>hi</p>".into(),
            preview_body: "<p>hi</p>".into(),
            attachments: vec![],
        }
    }

    fn setup(
        n: usize,
        sender: RecordingSender,
    ) -> (EmailUseCases, Arc<RecordingSender>, Arc<InMemoryEmails>) {
        let sender = Arc::new(sender);
        let emails = Arc::new(InMemoryEmails::default());
        let members = Arc::new(Recipients((0..n).map(|i| format!("m{i}@x.com")).collect()));
        let uc = EmailUseCases::new(
            emails.clone(),
            Arc::new(FixedSettings),
            members,
            vec![sender.clone()],
            Clock::new(chrono_tz::UTC),
        );
        (uc, sender, emails)
    }

    #[tokio::test]
    async fn bulk_sends_are_bcc_batches() {
        let (uc, sender, _) = setup(60, RecordingSender::default());
        let email = uc.send_to_all(content()).await.unwrap();
        assert_eq!(email.recipients.len(), 60);
        let sent = sender.sent.lock().unwrap();
        assert_eq!(sent.len(), 2);
        assert_eq!(sent[0].0.len(), MAX_RECIPIENTS_PER_MESSAGE);
        assert!(sent.iter().all(|(_, hidden)| *hidden));
    }

    #[tokio::test]
    async fn single_recipient_is_not_hidden() {
        let (uc, sender, _) = setup(1, RecordingSender::default());
        uc.send_to_member(Uuid::nil(), content()).await.unwrap();
        assert!(!sender.sent.lock().unwrap()[0].1);
    }

    #[tokio::test]
    async fn daily_limit_is_enforced() {
        let (uc, _, _) = setup(60, RecordingSender::default());
        uc.send_to_all(content()).await.unwrap();
        assert!(matches!(
            uc.send_to_all(content()).await,
            Err(AppError::TooManyRequests(_))
        ));
        let status = uc.provider_status().await.unwrap();
        assert_eq!((status.sent_today, status.remaining), (60, 40));
    }

    #[tokio::test]
    async fn partial_failure_records_what_was_delivered() {
        let sender = RecordingSender {
            fail_after: Some(1),
            ..Default::default()
        };
        let (uc, _, emails) = setup(60, sender);
        assert!(matches!(
            uc.send_to_all(content()).await,
            Err(AppError::EmailSend(_))
        ));
        let saved = emails.saved.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].recipients.len(), MAX_RECIPIENTS_PER_MESSAGE);
    }
}
