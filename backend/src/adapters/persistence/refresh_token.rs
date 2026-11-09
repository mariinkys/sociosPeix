use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence, app_error::AppResult,
    entities::refresh_token::RefreshToken, use_cases::auth::RefreshTokenPersistence,
};

#[derive(sqlx::FromRow)]
struct RefreshTokenDb {
    id: Uuid,
    token: String,
    user_email: String,
    expires_at: NaiveDateTime,
    revoked: bool,
    created_at: NaiveDateTime,
}

impl From<RefreshTokenDb> for RefreshToken {
    fn from(t: RefreshTokenDb) -> Self {
        RefreshToken {
            id: t.id,
            token: t.token,
            user_email: t.user_email,
            expires_at: t.expires_at,
            revoked: t.revoked,
            created_at: t.created_at,
        }
    }
}

#[async_trait]
impl RefreshTokenPersistence for PostgresPersistence {
    async fn create_refresh_token(
        &self,
        token: &str,
        user_email: &str,
        expires_at: NaiveDateTime,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO refresh_tokens (token, user_email, expires_at, created_at)
             VALUES ($1, $2, $3, (now() AT TIME ZONE 'UTC'))",
        )
        .bind(token)
        .bind(user_email)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_refresh_token(&self, token: &str) -> AppResult<Option<RefreshToken>> {
        let row = sqlx::query_as::<_, RefreshTokenDb>(
            "SELECT id, token, user_email, expires_at, revoked, created_at
             FROM refresh_tokens WHERE token = $1",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(RefreshToken::from))
    }

    async fn revoke_refresh_token(&self, id: Uuid) -> AppResult<bool> {
        let result =
            sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE id = $1 AND NOT revoked")
                .bind(id)
                .execute(&self.pool)
                .await?;
        Ok(result.rows_affected() == 1)
    }

    async fn revoke_all_refresh_tokens(&self, user_email: &str) -> AppResult<()> {
        sqlx::query(
            "UPDATE refresh_tokens SET revoked = true WHERE user_email = $1 AND NOT revoked",
        )
        .bind(user_email)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expired_refresh_tokens(&self, now: NaiveDateTime) -> AppResult<u64> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < $1")
            .bind(now)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
