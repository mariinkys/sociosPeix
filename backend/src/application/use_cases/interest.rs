use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tracing::instrument;

use crate::{
    app_error::{AppError, AppResult},
    entities::interest::Interest,
};

#[async_trait]
pub trait InterestPersistence: Send + Sync {
    async fn list_interests(&self) -> AppResult<Vec<Interest>>;
    async fn find_interest(&self, id: i32) -> AppResult<Option<Interest>>;
    /// Fails with [`AppError::Conflict`] if the name is taken.
    async fn create_interest(&self, name: &str, description: &str) -> AppResult<Interest>;
    /// Returns `None` if the interest doesn't exist.
    async fn update_interest(
        &self,
        id: i32,
        name: &str,
        description: &str,
    ) -> AppResult<Option<Interest>>;
    /// Returns `false` if the interest doesn't exist.
    async fn delete_interest(&self, id: i32) -> AppResult<bool>;
}

/// The full interest list is requested on almost every screen, so it is
/// cached in memory and invalidated on every write.
pub struct InterestUseCases {
    persistence: Arc<dyn InterestPersistence>,
    cache: RwLock<Cache>,
}

/// `generation` is bumped on every write so a read that raced with a write
/// never stores a stale list.
#[derive(Default)]
struct Cache {
    generation: u64,
    interests: Option<Arc<[Interest]>>,
}

fn not_found(id: i32) -> AppError {
    AppError::NotFound(format!("Interest not found with id: {id}"))
}

impl InterestUseCases {
    pub fn new(persistence: Arc<dyn InterestPersistence>) -> Self {
        Self {
            persistence,
            cache: RwLock::new(Cache::default()),
        }
    }

    pub async fn list(&self) -> AppResult<Arc<[Interest]>> {
        let generation = {
            let cache = self.cache.read().expect("cache lock");
            if let Some(cached) = &cache.interests {
                return Ok(cached.clone());
            }
            cache.generation
        };
        let fresh: Arc<[Interest]> = self.persistence.list_interests().await?.into();
        let mut cache = self.cache.write().expect("cache lock");
        if cache.generation == generation {
            cache.interests = Some(fresh.clone());
        }
        Ok(fresh)
    }

    pub async fn get(&self, id: i32) -> AppResult<Interest> {
        self.persistence
            .find_interest(id)
            .await?
            .ok_or_else(|| not_found(id))
    }

    #[instrument(skip(self))]
    pub async fn create(&self, name: &str, description: &str) -> AppResult<Interest> {
        let created = self.persistence.create_interest(name, description).await?;
        self.invalidate();
        Ok(created)
    }

    #[instrument(skip(self))]
    pub async fn update(&self, id: i32, name: &str, description: &str) -> AppResult<Interest> {
        let updated = self
            .persistence
            .update_interest(id, name, description)
            .await?
            .ok_or_else(|| not_found(id))?;
        self.invalidate();
        Ok(updated)
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: i32) -> AppResult<()> {
        if !self.persistence.delete_interest(id).await? {
            return Err(not_found(id));
        }
        self.invalidate();
        Ok(())
    }

    fn invalidate(&self) {
        let mut cache = self.cache.write().expect("cache lock");
        cache.generation += 1;
        cache.interests = None;
    }
}
