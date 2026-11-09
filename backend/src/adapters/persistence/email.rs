use std::collections::HashMap;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::AppResult,
    domain::pagination::{Page, PageRequest},
    entities::email::{Email, EmailCategory, NewEmail},
    use_cases::email::{EmailPersistence, EmailSettingsPersistence},
};

#[derive(sqlx::FromRow)]
struct EmailDb {
    id: Uuid,
    subject: String,
    body: String,
    provider: String,
    category: String,
    created_at: NaiveDateTime,
}

impl PostgresPersistence {
    /// Attaches recipients to a batch of emails with a single query.
    async fn with_recipients(&self, rows: Vec<EmailDb>) -> AppResult<Vec<Email>> {
        let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let mut recipients: HashMap<Uuid, Vec<String>> = HashMap::with_capacity(ids.len());
        if !ids.is_empty() {
            let pairs: Vec<(Uuid, String)> = sqlx::query_as(
                "SELECT email_id, email FROM email_recipients WHERE email_id = ANY($1)",
            )
            .bind(&ids)
            .fetch_all(&self.pool)
            .await?;
            for (id, email) in pairs {
                recipients.entry(id).or_default().push(email);
            }
        }
        Ok(rows
            .into_iter()
            .map(|r| Email {
                recipients: recipients.remove(&r.id).unwrap_or_default(),
                id: r.id,
                subject: r.subject,
                body: r.body,
                provider: r.provider,
                category: EmailCategory::parse(&r.category),
                created_at: r.created_at,
            })
            .collect())
    }
}

#[async_trait]
impl EmailPersistence for PostgresPersistence {
    async fn save_email(&self, e: &NewEmail) -> AppResult<Email> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, EmailDb>(
            "INSERT INTO emails (subject, body, provider, category, created_at)
             VALUES ($1, $2, $3, $4, (now() AT TIME ZONE 'UTC'))
             RETURNING id, subject, body, provider, category, created_at",
        )
        .bind(&e.subject)
        .bind(&e.body)
        .bind(&e.provider)
        .bind(e.category.as_str())
        .fetch_one(&mut *tx)
        .await?;
        sqlx::query("INSERT INTO email_recipients (email_id, email) SELECT $1, unnest($2::text[])")
            .bind(row.id)
            .bind(&e.recipients)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;

        Ok(Email {
            id: row.id,
            subject: row.subject,
            body: row.body,
            provider: row.provider,
            recipients: e.recipients.clone(),
            category: e.category,
            created_at: row.created_at,
        })
    }

    async fn find_email(&self, id: Uuid) -> AppResult<Option<Email>> {
        let Some(row) = sqlx::query_as::<_, EmailDb>(
            "SELECT id, subject, body, provider, category, created_at FROM emails WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        else {
            return Ok(None);
        };
        Ok(self.with_recipients(vec![row]).await?.pop())
    }

    async fn list_emails(&self, page: PageRequest) -> AppResult<Page<Email>> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM emails")
            .fetch_one(&self.pool)
            .await?;
        let rows = sqlx::query_as::<_, EmailDb>(
            "SELECT id, subject, body, provider, category, created_at FROM emails
             ORDER BY created_at DESC, id LIMIT $1 OFFSET $2",
        )
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(&self.pool)
        .await?;
        Ok(Page {
            items: self.with_recipients(rows).await?,
            request: page,
            total,
        })
    }

    async fn list_emails_since(&self, since: NaiveDateTime) -> AppResult<Vec<Email>> {
        let rows = sqlx::query_as::<_, EmailDb>(
            "SELECT id, subject, body, provider, category, created_at FROM emails
             WHERE created_at >= $1 ORDER BY created_at DESC",
        )
        .bind(since)
        .fetch_all(&self.pool)
        .await?;
        self.with_recipients(rows).await
    }

    async fn list_emails_for_member(
        &self,
        member_id: Uuid,
        page: PageRequest,
    ) -> AppResult<Page<Email>> {
        const FILTER: &str = "EXISTS (SELECT 1 FROM email_recipients r
                                      JOIN members m ON m.email = r.email
                                      WHERE r.email_id = e.id AND m.id = $1)";
        let total: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(format!(
            "SELECT COUNT(*) FROM emails e WHERE {FILTER}"
        )))
        .bind(member_id)
        .fetch_one(&self.pool)
        .await?;
        let rows = sqlx::query_as::<_, EmailDb>(sqlx::AssertSqlSafe(format!(
            "SELECT e.id, e.subject, e.body, e.provider, e.category, e.created_at
             FROM emails e WHERE {FILTER}
             ORDER BY e.created_at DESC, e.id LIMIT $2 OFFSET $3"
        )))
        .bind(member_id)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(&self.pool)
        .await?;
        Ok(Page {
            items: self.with_recipients(rows).await?,
            request: page,
            total,
        })
    }

    async fn count_recipients_since(&self, provider: &str, since: NaiveDateTime) -> AppResult<i64> {
        Ok(sqlx::query_scalar(
            "SELECT COUNT(*) FROM email_recipients r JOIN emails e ON e.id = r.email_id
             WHERE e.provider = $1 AND e.created_at >= $2",
        )
        .bind(provider)
        .bind(since)
        .fetch_one(&self.pool)
        .await?)
    }

    async fn was_sent_since(
        &self,
        recipient: &str,
        subject: &str,
        since: NaiveDateTime,
    ) -> AppResult<bool> {
        Ok(sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM emails e JOIN email_recipients r ON r.email_id = e.id
                            WHERE e.subject = $2 AND r.email = $1 AND e.created_at >= $3)",
        )
        .bind(recipient)
        .bind(subject)
        .bind(since)
        .fetch_one(&self.pool)
        .await?)
    }
}

const SETTINGS_ROW_ID: i32 = 1;

#[async_trait]
impl EmailSettingsPersistence for PostgresPersistence {
    async fn active_provider(&self) -> AppResult<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT active_provider FROM email_settings WHERE id = $1")
                .bind(SETTINGS_ROW_ID)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    async fn set_active_provider(&self, name: &str) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO email_settings (id, active_provider) VALUES ($1, $2)
             ON CONFLICT (id) DO UPDATE SET active_provider = EXCLUDED.active_provider",
        )
        .bind(SETTINGS_ROW_ID)
        .bind(name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
