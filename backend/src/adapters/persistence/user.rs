use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::AssertSqlSafe;
use uuid::Uuid;

use crate::{
    adapters::persistence::{PostgresPersistence, conflict_on_unique, like_pattern},
    app_error::{AppError, AppResult},
    domain::pagination::{Page, PageRequest, SortDirection},
    entities::user::{Role, User},
    use_cases::user::{UserPersistence, UserSortField},
};

#[derive(sqlx::FromRow)]
struct UserDb {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    role: String,
    failed_login_attempts: i32,
    locked_until: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<UserDb> for User {
    fn from(u: UserDb) -> Self {
        User {
            id: u.id,
            name: u.name,
            email: u.email,
            password_hash: u.password,
            role: Role::parse(&u.role).unwrap_or(Role::User),
            failed_login_attempts: u.failed_login_attempts,
            locked_until: u.locked_until,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

const COLUMNS: &str =
    "id, name, email, password, role, failed_login_attempts, locked_until, created_at, updated_at";

fn email_taken(email: &str) -> impl FnOnce() -> String + '_ {
    move || format!("Email already in use: {email}")
}

fn not_found(id: Uuid) -> AppError {
    AppError::NotFound(format!("User not found with id: {id}"))
}

fn sort_column(field: UserSortField) -> &'static str {
    match field {
        UserSortField::Name => "name",
        UserSortField::Email => "email",
        UserSortField::Role => "role",
        UserSortField::CreatedAt => "created_at",
    }
}

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let row = sqlx::query_as::<_, UserDb>(
            "SELECT id, name, email, password, role, failed_login_attempts, locked_until, created_at, updated_at
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(User::from))
    }

    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let row = sqlx::query_as::<_, UserDb>(
            "SELECT id, name, email, password, role, failed_login_attempts, locked_until, created_at, updated_at
             FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(User::from))
    }

    async fn list_users(
        &self,
        search: &str,
        (field, dir): (UserSortField, SortDirection),
        page: PageRequest,
    ) -> AppResult<Page<User>> {
        const FILTER: &str = "($1 = '' OR name ILIKE $2 OR email ILIKE $2)";
        let pattern = like_pattern(search);

        let total: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "SELECT COUNT(*) FROM users WHERE {FILTER}"
        )))
        .bind(search)
        .bind(&pattern)
        .fetch_one(&self.pool)
        .await?;

        // The ORDER BY column comes from a closed enum, never from user input.
        let sql = format!(
            "SELECT {COLUMNS} FROM users WHERE {FILTER}
             ORDER BY {} {}, id LIMIT $3 OFFSET $4",
            sort_column(field),
            dir.as_sql()
        );
        let rows = sqlx::query_as::<_, UserDb>(AssertSqlSafe(sql))
            .bind(search)
            .bind(&pattern)
            .bind(page.limit())
            .bind(page.offset())
            .fetch_all(&self.pool)
            .await?;

        Ok(Page {
            items: rows.into_iter().map(User::from).collect(),
            request: page,
            total,
        })
    }

    async fn create_user(&self, name: &str, email: &str, password_hash: &str) -> AppResult<User> {
        sqlx::query_as::<_, UserDb>(
            "INSERT INTO users (name, email, password, role) VALUES ($1, $2, $3, 'USER')
             RETURNING id, name, email, password, role, failed_login_attempts, locked_until, created_at, updated_at",
        )
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map(User::from)
        .map_err(|e| conflict_on_unique(e, email_taken(email)))
    }

    async fn update_user_details(&self, id: Uuid, name: &str, email: &str) -> AppResult<User> {
        sqlx::query_as::<_, UserDb>(
            "UPDATE users SET name = $2, email = $3, updated_at = (now() AT TIME ZONE 'UTC')
             WHERE id = $1
             RETURNING id, name, email, password, role, failed_login_attempts, locked_until, created_at, updated_at",
        )
        .bind(id)
        .bind(name)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| conflict_on_unique(e, email_taken(email)))?
        .map(User::from)
        .ok_or_else(|| not_found(id))
    }

    async fn update_user_role(&self, id: Uuid, role: Role) -> AppResult<User> {
        sqlx::query_as::<_, UserDb>(
            "UPDATE users SET role = $2, updated_at = (now() AT TIME ZONE 'UTC')
             WHERE id = $1
             RETURNING id, name, email, password, role, failed_login_attempts, locked_until, created_at, updated_at",
        )
        .bind(id)
        .bind(role.as_str())
        .fetch_optional(&self.pool)
        .await?
        .map(User::from)
        .ok_or_else(|| not_found(id))
    }

    async fn update_user_password(&self, id: Uuid, password_hash: &str) -> AppResult<()> {
        sqlx::query(
            "UPDATE users SET password = $2, updated_at = (now() AT TIME ZONE 'UTC') WHERE id = $1",
        )
        .bind(id)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn reset_failed_logins(&self, id: Uuid) -> AppResult<()> {
        sqlx::query(
            "UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn record_failed_login(
        &self,
        id: Uuid,
        attempts: i32,
        locked_until: Option<NaiveDateTime>,
    ) -> AppResult<()> {
        sqlx::query("UPDATE users SET failed_login_attempts = $2, locked_until = $3 WHERE id = $1")
            .bind(id)
            .bind(attempts)
            .bind(locked_until)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_user(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
