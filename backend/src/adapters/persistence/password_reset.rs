use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence, app_error::AppResult,
    entities::password_reset_code::PasswordResetCode,
    use_cases::password_reset::PasswordResetPersistence,
};

#[derive(sqlx::FromRow)]
struct ResetCodeDb {
    id: Uuid,
    email: String,
    code_hash: String,
    expires_at: NaiveDateTime,
    created_at: NaiveDateTime,
    attempts: i32,
    used: bool,
}

impl From<ResetCodeDb> for PasswordResetCode {
    fn from(c: ResetCodeDb) -> Self {
        PasswordResetCode {
            id: c.id,
            email: c.email,
            code_hash: c.code_hash,
            expires_at: c.expires_at,
            created_at: c.created_at,
            attempts: c.attempts,
            used: c.used,
        }
    }
}

#[async_trait]
impl PasswordResetPersistence for PostgresPersistence {
    async fn latest_reset_code(&self, email: &str) -> AppResult<Option<PasswordResetCode>> {
        let row = sqlx::query_as::<_, ResetCodeDb>(
            "SELECT id, email, code_hash, expires_at, created_at, attempts, used
             FROM password_reset_codes WHERE email = $1
             ORDER BY created_at DESC LIMIT 1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(PasswordResetCode::from))
    }

    async fn replace_reset_code(
        &self,
        email: &str,
        code_hash: &str,
        expires_at: NaiveDateTime,
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM password_reset_codes WHERE email = $1")
            .bind(email)
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO password_reset_codes (email, code_hash, expires_at, created_at)
             VALUES ($1, $2, $3, (now() AT TIME ZONE 'UTC'))",
        )
        .bind(email)
        .bind(code_hash)
        .bind(expires_at)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn register_failed_reset_attempt(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE password_reset_codes SET attempts = attempts + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_reset_code_used(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE password_reset_codes SET used = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_reset_codes_expired_before(&self, cutoff: NaiveDateTime) -> AppResult<u64> {
        let result = sqlx::query("DELETE FROM password_reset_codes WHERE expires_at < $1")
            .bind(cutoff)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
