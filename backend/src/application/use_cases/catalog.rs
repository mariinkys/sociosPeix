use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::OnceCell;

use crate::{
    app_error::AppResult,
    entities::catalog::{Country, Gender},
};

#[async_trait]
pub trait CatalogPersistence: Send + Sync {
    async fn list_genders(&self) -> AppResult<Vec<Gender>>;
    async fn list_countries(&self) -> AppResult<Vec<Country>>;
}

/// Genders and countries are seed data that never change at runtime, so they
/// are read from the database once and then served from memory.
pub struct CatalogUseCases {
    persistence: Arc<dyn CatalogPersistence>,
    genders: OnceCell<Arc<[Gender]>>,
    countries: OnceCell<Arc<[Country]>>,
}

impl CatalogUseCases {
    pub fn new(persistence: Arc<dyn CatalogPersistence>) -> Self {
        Self {
            persistence,
            genders: OnceCell::new(),
            countries: OnceCell::new(),
        }
    }

    pub async fn genders(&self) -> AppResult<Arc<[Gender]>> {
        self.genders
            .get_or_try_init(|| async { Ok(self.persistence.list_genders().await?.into()) })
            .await
            .cloned()
    }

    pub async fn countries(&self) -> AppResult<Arc<[Country]>> {
        self.countries
            .get_or_try_init(|| async { Ok(self.persistence.list_countries().await?.into()) })
            .await
            .cloned()
    }

    pub async fn gender_exists(&self, id: i32) -> AppResult<bool> {
        Ok(self.genders().await?.iter().any(|g| g.id == id))
    }

    pub async fn country_exists(&self, id: i32) -> AppResult<bool> {
        Ok(self.countries().await?.iter().any(|c| c.id == id))
    }
}
