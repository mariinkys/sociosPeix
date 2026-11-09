use async_trait::async_trait;

use crate::{
    adapters::persistence::{PostgresPersistence, conflict_on_unique},
    app_error::AppResult,
    entities::interest::Interest,
    use_cases::interest::InterestPersistence,
};

#[derive(sqlx::FromRow)]
pub(crate) struct InterestDb {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<InterestDb> for Interest {
    fn from(i: InterestDb) -> Self {
        Interest {
            id: i.id,
            name: i.name,
            description: i.description,
        }
    }
}

fn name_taken(name: &str) -> impl FnOnce() -> String + '_ {
    move || format!("An interest named '{name}' already exists")
}

#[async_trait]
impl InterestPersistence for PostgresPersistence {
    async fn list_interests(&self) -> AppResult<Vec<Interest>> {
        let rows = sqlx::query_as::<_, InterestDb>(
            "SELECT id, name, description FROM interests ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(Interest::from).collect())
    }

    async fn find_interest(&self, id: i32) -> AppResult<Option<Interest>> {
        let row = sqlx::query_as::<_, InterestDb>(
            "SELECT id, name, description FROM interests WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(Interest::from))
    }

    async fn create_interest(&self, name: &str, description: &str) -> AppResult<Interest> {
        sqlx::query_as::<_, InterestDb>(
            "INSERT INTO interests (name, description) VALUES ($1, $2)
             RETURNING id, name, description",
        )
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await
        .map(Interest::from)
        .map_err(|e| conflict_on_unique(e, name_taken(name)))
    }

    async fn update_interest(
        &self,
        id: i32,
        name: &str,
        description: &str,
    ) -> AppResult<Option<Interest>> {
        let row = sqlx::query_as::<_, InterestDb>(
            "UPDATE interests SET name = $2, description = $3 WHERE id = $1
             RETURNING id, name, description",
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| conflict_on_unique(e, name_taken(name)))?;
        Ok(row.map(Interest::from))
    }

    async fn delete_interest(&self, id: i32) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM interests WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }
}
