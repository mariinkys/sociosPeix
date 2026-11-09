use async_trait::async_trait;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::AppResult,
    entities::catalog::{Country, Gender},
    use_cases::catalog::CatalogPersistence,
};

#[async_trait]
impl CatalogPersistence for PostgresPersistence {
    async fn list_genders(&self) -> AppResult<Vec<Gender>> {
        let rows: Vec<(i32, String)> = sqlx::query_as("SELECT id, name FROM genders ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .into_iter()
            .map(|(id, name)| Gender { id, name })
            .collect())
    }

    async fn list_countries(&self) -> AppResult<Vec<Country>> {
        let rows: Vec<(i32, String)> = sqlx::query_as("SELECT id, name FROM countries ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .into_iter()
            .map(|(id, name)| Country { id, name })
            .collect())
    }
}
