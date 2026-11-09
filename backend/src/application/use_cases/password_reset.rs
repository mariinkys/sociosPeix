use std::sync::Arc;

use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{Duration, NaiveDateTime};
use rand::RngExt;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    clock::Clock,
    entities::password_reset_code::PasswordResetCode,
    use_cases::{
        auth::RefreshTokenPersistence,
        email::EmailUseCases,
        user::{PasswordHasher, UserPersistence},
    },
};

#[async_trait]
pub trait PasswordResetPersistence: Send + Sync {
    /// The most recently issued code for this email, if any.
    async fn latest_reset_code(&self, email: &str) -> AppResult<Option<PasswordResetCode>>;
    /// Deletes every previous code for the email and stores the new one.
    async fn replace_reset_code(
        &self,
        email: &str,
        code_hash: &str,
        expires_at: NaiveDateTime,
    ) -> AppResult<()>;
    async fn register_failed_reset_attempt(&self, id: Uuid) -> AppResult<()>;
    async fn mark_reset_code_used(&self, id: Uuid) -> AppResult<()>;
    async fn delete_reset_codes_expired_before(&self, cutoff: NaiveDateTime) -> AppResult<u64>;
}

const CODE_TTL_MINUTES: i64 = 15;
const RESEND_COOLDOWN_MINUTES: i64 = 5;

pub struct PasswordResetUseCases {
    users: Arc<dyn UserPersistence>,
    codes: Arc<dyn PasswordResetPersistence>,
    hasher: Arc<dyn PasswordHasher>,
    refresh_tokens: Arc<dyn RefreshTokenPersistence>,
    emails: Arc<EmailUseCases>,
    clock: Clock,
}

fn invalid_code() -> AppError {
    AppError::Validation("Invalid, expired, or already-used reset code".into())
}

impl PasswordResetUseCases {
    pub fn new(
        users: Arc<dyn UserPersistence>,
        codes: Arc<dyn PasswordResetPersistence>,
        hasher: Arc<dyn PasswordHasher>,
        refresh_tokens: Arc<dyn RefreshTokenPersistence>,
        emails: Arc<EmailUseCases>,
        clock: Clock,
    ) -> Self {
        Self {
            users,
            codes,
            hasher,
            refresh_tokens,
            emails,
            clock,
        }
    }

    /// Emails a 6-digit code. The caller always gets the same response —
    /// whether or not the account exists, or a code was sent moments ago — so
    /// this can't be used to discover accounts. The email itself is sent in
    /// the background to keep response timing identical too.
    #[instrument(skip(self))]
    pub async fn request_reset(&self, email: &str) -> AppResult<()> {
        if self.users.find_user_by_email(email).await?.is_none() {
            info!("password reset requested for an email with no account");
            return Ok(());
        }

        let now = self.clock.now();
        if let Some(existing) = self.codes.latest_reset_code(email).await?
            && existing.created_at + Duration::minutes(RESEND_COOLDOWN_MINUTES) > now
        {
            info!("password reset requested again within the cooldown; ignoring");
            return Ok(());
        }

        let code = generate_code();
        self.codes
            .replace_reset_code(
                email,
                &hash_code(&code),
                now + Duration::minutes(CODE_TTL_MINUTES),
            )
            .await?;

        let emails = self.emails.clone();
        let email = email.to_owned();
        tokio::spawn(async move {
            if let Err(e) = emails
                .send_transactional(&email, "Your password reset code", &reset_email_html(&code))
                .await
            {
                error!(error = %e, "failed to send password reset email");
            }
        });
        Ok(())
    }

    #[instrument(skip(self, code, new_password))]
    pub async fn reset_password(
        &self,
        email: &str,
        code: &str,
        new_password: &str,
    ) -> AppResult<()> {
        let reset = self
            .codes
            .latest_reset_code(email)
            .await?
            .filter(|c| c.is_usable(self.clock.now()))
            .ok_or_else(invalid_code)?;

        let matches: bool = hash_code(code)
            .as_bytes()
            .ct_eq(reset.code_hash.as_bytes())
            .into();
        if !matches {
            self.codes.register_failed_reset_attempt(reset.id).await?;
            return Err(invalid_code());
        }

        let user = self
            .users
            .find_user_by_email(email)
            .await?
            .ok_or_else(invalid_code)?;
        let hash = self.hasher.hash(new_password).await?;
        self.users.update_user_password(user.id, &hash).await?;
        self.codes.mark_reset_code_used(reset.id).await?;
        // Log out every existing session.
        self.refresh_tokens.revoke_all_refresh_tokens(email).await?;
        info!(user_id = %user.id, "password reset completed");
        Ok(())
    }

    /// Housekeeping: drop codes that expired more than a day ago.
    pub async fn delete_expired_codes(&self) -> AppResult<u64> {
        self.codes
            .delete_reset_codes_expired_before(self.clock.now() - Duration::days(1))
            .await
    }
}

fn generate_code() -> String {
    format!("{:06}", rand::rng().random_range(0..1_000_000u32))
}

/// Base64(SHA-256(code)) — same format the previous backend stored.
fn hash_code(code: &str) -> String {
    STANDARD.encode(Sha256::digest(code.as_bytes()))
}

fn reset_email_html(code: &str) -> String {
    format!(
        "<p>Use the code below to reset your password. It expires in {CODE_TTL_MINUTES} minutes.</p>\n\
         <h2>{code}</h2>\n\
         <p>If you didn't request this, you can safely ignore this email.</p>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_format_and_hash_are_stable() {
        for _ in 0..100 {
            let c = generate_code();
            assert_eq!(c.len(), 6);
            assert!(c.chars().all(|ch| ch.is_ascii_digit()));
        }
        // Must match what the Java backend produced for existing rows.
        assert_eq!(
            hash_code("123456"),
            "jZae727K08KaOmKSgOaGzww/XVqGr/PKEgIMkjrcbJI="
        );
    }
}
