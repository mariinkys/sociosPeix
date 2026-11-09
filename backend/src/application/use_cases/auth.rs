use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime};
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    clock::Clock,
    entities::{refresh_token::RefreshToken, user::User},
    use_cases::user::{PasswordHasher, UserPersistence},
};

#[async_trait]
pub trait RefreshTokenPersistence: Send + Sync {
    async fn create_refresh_token(
        &self,
        token: &str,
        user_email: &str,
        expires_at: NaiveDateTime,
    ) -> AppResult<()>;
    async fn find_refresh_token(&self, token: &str) -> AppResult<Option<RefreshToken>>;
    /// Atomically revokes the token; returns `false` if it was already revoked
    /// (i.e. another request rotated it first).
    async fn revoke_refresh_token(&self, id: Uuid) -> AppResult<bool>;
    async fn revoke_all_refresh_tokens(&self, user_email: &str) -> AppResult<()>;
    async fn delete_expired_refresh_tokens(&self, now: NaiveDateTime) -> AppResult<u64>;
}

/// Issues and verifies the short-lived access token (a JWT).
pub trait AccessTokenIssuer: Send + Sync {
    fn issue(&self, email: &str) -> AppResult<String>;
    /// Returns the subject (email) if the token is authentic and unexpired.
    fn verify(&self, token: &str) -> Option<String>;
}

#[derive(Debug, Clone, Copy)]
pub struct AuthSettings {
    pub max_login_attempts: i32,
    pub lockout_duration: Duration,
    pub refresh_token_ttl: Duration,
}

/// Both tokens of a session.
#[derive(Debug)]
pub struct SessionTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthUseCases {
    users: Arc<dyn UserPersistence>,
    hasher: Arc<dyn PasswordHasher>,
    refresh_tokens: Arc<dyn RefreshTokenPersistence>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    settings: AuthSettings,
    clock: Clock,
    /// A real bcrypt hash used to keep login timing the same whether or not
    /// the account exists.
    dummy_hash: String,
}

pub const INVALID_CREDENTIALS: &str = "Invalid email or password";

impl AuthUseCases {
    pub fn new(
        users: Arc<dyn UserPersistence>,
        hasher: Arc<dyn PasswordHasher>,
        refresh_tokens: Arc<dyn RefreshTokenPersistence>,
        access_tokens: Arc<dyn AccessTokenIssuer>,
        settings: AuthSettings,
        clock: Clock,
        dummy_hash: String,
    ) -> Self {
        Self {
            users,
            hasher,
            refresh_tokens,
            access_tokens,
            settings,
            clock,
            dummy_hash,
        }
    }

    #[instrument(skip(self, password))]
    pub async fn login(&self, email: &str, password: &str) -> AppResult<(User, SessionTokens)> {
        let now = self.clock.now();
        let Some(user) = self.users.find_user_by_email(email).await? else {
            // Burn the same CPU time as a real check so response timing
            // doesn't reveal which emails have accounts.
            let _ = self.hasher.verify(password, &self.dummy_hash).await;
            return Err(AppError::Unauthorized(INVALID_CREDENTIALS.into()));
        };

        if user.is_locked(now) {
            let minutes = self.settings.lockout_duration.num_minutes();
            return Err(AppError::Forbidden(format!(
                "Account locked due to too many failed attempts. Try again in {minutes} minutes."
            )));
        }

        if !self.hasher.verify(password, &user.password_hash).await {
            self.register_failure(&user, now).await?;
            return Err(AppError::Unauthorized(INVALID_CREDENTIALS.into()));
        }

        if user.failed_login_attempts > 0 || user.locked_until.is_some() {
            self.users.reset_failed_logins(user.id).await?;
        }

        let tokens = self.issue_session(&user.email).await?;
        info!(user_id = %user.id, "login succeeded");
        Ok((user, tokens))
    }

    async fn register_failure(&self, user: &User, now: NaiveDateTime) -> AppResult<()> {
        let attempts = user.effective_failed_attempts(now) + 1;
        let locked_until = if attempts >= self.settings.max_login_attempts {
            let until = now + self.settings.lockout_duration;
            warn!(user_id = %user.id, %until, "account locked after too many failed logins");
            Some(until)
        } else {
            warn!(
                user_id = %user.id,
                attempts,
                max = self.settings.max_login_attempts,
                "failed login attempt"
            );
            None
        };
        self.users
            .record_failed_login(user.id, attempts, locked_until)
            .await
    }

    /// Validates and rotates a refresh token, returning a fresh session.
    ///
    /// Presenting an already-revoked token means it was stolen or replayed, so
    /// every session of that user is revoked.
    #[instrument(skip_all)]
    pub async fn refresh(&self, refresh_token: &str) -> AppResult<(User, SessionTokens)> {
        let token = self
            .refresh_tokens
            .find_refresh_token(refresh_token)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Refresh token not found".into()))?;

        if token.revoked || !self.refresh_tokens.revoke_refresh_token(token.id).await? {
            warn!(email = %token.user_email, "refresh token reuse detected");
            self.refresh_tokens
                .revoke_all_refresh_tokens(&token.user_email)
                .await?;
            return Err(AppError::Unauthorized(
                "Token reuse detected — all sessions revoked".into(),
            ));
        }
        if token.is_expired(self.clock.now()) {
            return Err(AppError::Unauthorized("Refresh token has expired".into()));
        }

        let user = self
            .users
            .find_user_by_email(&token.user_email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Account no longer exists".into()))?;

        let tokens = self.issue_session(&user.email).await?;
        Ok((user, tokens))
    }

    pub async fn logout(&self, email: &str) -> AppResult<()> {
        self.refresh_tokens.revoke_all_refresh_tokens(email).await
    }

    /// Resolves the user behind an access token.
    pub async fn authenticate(&self, access_token: &str) -> AppResult<User> {
        let email = self
            .access_tokens
            .verify(access_token)
            .ok_or(AppError::Unauthenticated)?;
        self.users
            .find_user_by_email(&email)
            .await?
            .ok_or(AppError::Unauthenticated)
    }

    pub async fn revoke_all_sessions(&self, email: &str) -> AppResult<()> {
        self.refresh_tokens.revoke_all_refresh_tokens(email).await
    }

    pub async fn delete_expired_tokens(&self) -> AppResult<u64> {
        self.refresh_tokens
            .delete_expired_refresh_tokens(self.clock.now())
            .await
    }

    async fn issue_session(&self, email: &str) -> AppResult<SessionTokens> {
        let access_token = self.access_tokens.issue(email)?;
        let refresh_token = Uuid::new_v4().to_string();
        let expires_at = self.clock.now() + self.settings.refresh_token_ttl;
        self.refresh_tokens
            .create_refresh_token(&refresh_token, email, expires_at)
            .await?;
        Ok(SessionTokens {
            access_token,
            refresh_token,
        })
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    pub struct InMemoryRefreshTokens {
        pub tokens: Mutex<Vec<RefreshToken>>,
    }

    #[async_trait]
    impl RefreshTokenPersistence for InMemoryRefreshTokens {
        async fn create_refresh_token(
            &self,
            token: &str,
            user_email: &str,
            expires_at: NaiveDateTime,
        ) -> AppResult<()> {
            self.tokens.lock().unwrap().push(RefreshToken {
                id: Uuid::new_v4(),
                token: token.into(),
                user_email: user_email.into(),
                expires_at,
                revoked: false,
                created_at: chrono::Utc::now().naive_utc(),
            });
            Ok(())
        }
        async fn find_refresh_token(&self, token: &str) -> AppResult<Option<RefreshToken>> {
            Ok(self
                .tokens
                .lock()
                .unwrap()
                .iter()
                .find(|t| t.token == token)
                .cloned())
        }
        async fn revoke_refresh_token(&self, id: Uuid) -> AppResult<bool> {
            let mut tokens = self.tokens.lock().unwrap();
            let t = tokens.iter_mut().find(|t| t.id == id).unwrap();
            let was_active = !t.revoked;
            t.revoked = true;
            Ok(was_active)
        }
        async fn revoke_all_refresh_tokens(&self, user_email: &str) -> AppResult<()> {
            for t in self.tokens.lock().unwrap().iter_mut() {
                if t.user_email == user_email {
                    t.revoked = true;
                }
            }
            Ok(())
        }
        async fn delete_expired_refresh_tokens(&self, now: NaiveDateTime) -> AppResult<u64> {
            let mut tokens = self.tokens.lock().unwrap();
            let before = tokens.len();
            tokens.retain(|t| t.expires_at >= now);
            Ok((before - tokens.len()) as u64)
        }
    }

    pub struct FakeJwt;

    impl AccessTokenIssuer for FakeJwt {
        fn issue(&self, email: &str) -> AppResult<String> {
            Ok(format!("jwt:{email}"))
        }
        fn verify(&self, token: &str) -> Option<String> {
            token.strip_prefix("jwt:").map(str::to_owned)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use crate::entities::user::Role;
    use crate::use_cases::user::test_support::*;

    fn setup() -> (AuthUseCases, Arc<InMemoryUsers>, Arc<InMemoryRefreshTokens>) {
        let users = Arc::new(InMemoryUsers::with(vec![user(
            "bob@x.com",
            Role::User,
            "hashed:secret123",
        )]));
        let tokens = Arc::new(InMemoryRefreshTokens::default());
        let uc = AuthUseCases::new(
            users.clone(),
            Arc::new(PlainHasher),
            tokens.clone(),
            Arc::new(FakeJwt),
            AuthSettings {
                max_login_attempts: 3,
                lockout_duration: Duration::minutes(15),
                refresh_token_ttl: Duration::days(7),
            },
            Clock::new(chrono_tz::UTC),
            "hashed:dummy".into(),
        );
        (uc, users, tokens)
    }

    #[tokio::test]
    async fn locks_after_max_attempts_and_rejects_correct_password() {
        let (uc, users, _) = setup();
        for _ in 0..3 {
            assert!(matches!(
                uc.login("bob@x.com", "nope").await,
                Err(AppError::Unauthorized(_))
            ));
        }
        let u = users
            .find_user_by_email("bob@x.com")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(u.failed_login_attempts, 3);
        assert!(u.locked_until.is_some());
        assert!(matches!(
            uc.login("bob@x.com", "secret123").await,
            Err(AppError::Forbidden(_))
        ));
    }

    #[tokio::test]
    async fn success_resets_counter_and_unknown_user_is_generic() {
        let (uc, users, _) = setup();
        let _ = uc.login("bob@x.com", "nope").await;
        let (user, tokens) = uc.login("bob@x.com", "secret123").await.unwrap();
        assert_eq!(tokens.access_token, "jwt:bob@x.com");
        let u = users.find_user_by_id(user.id).await.unwrap().unwrap();
        assert_eq!(u.failed_login_attempts, 0);

        match uc.login("ghost@x.com", "whatever").await {
            Err(AppError::Unauthorized(m)) => assert_eq!(m, INVALID_CREDENTIALS),
            other => panic!("unexpected {other:?}"),
        }
    }

    #[tokio::test]
    async fn refresh_rotates_and_detects_reuse() {
        let (uc, _, tokens) = setup();
        let (_, session) = uc.login("bob@x.com", "secret123").await.unwrap();
        let (_, rotated) = uc.refresh(&session.refresh_token).await.unwrap();
        assert_ne!(rotated.refresh_token, session.refresh_token);

        // Replaying the old one revokes everything, including the new token.
        assert!(uc.refresh(&session.refresh_token).await.is_err());
        assert!(tokens.tokens.lock().unwrap().iter().all(|t| t.revoked));
        assert!(uc.refresh(&rotated.refresh_token).await.is_err());
    }

    #[tokio::test]
    async fn authenticate_resolves_user() {
        let (uc, _, _) = setup();
        assert_eq!(
            uc.authenticate("jwt:bob@x.com").await.unwrap().email,
            "bob@x.com"
        );
        assert!(matches!(
            uc.authenticate("garbage").await,
            Err(AppError::Unauthenticated)
        ));
    }
}
