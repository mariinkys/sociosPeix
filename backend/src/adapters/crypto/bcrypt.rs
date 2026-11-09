use async_trait::async_trait;

use crate::{
    app_error::{AppError, AppResult},
    use_cases::user::PasswordHasher,
};

/// Same work factor Spring Security's `BCryptPasswordEncoder` used
const COST: u32 = 10;

/// bcrypt hasher. Hashing takes tens of milliseconds of pure CPU, so it runs
/// on the blocking thread pool instead of stalling the async workers.
#[derive(Default)]
pub struct BcryptPasswordHasher;

#[async_trait]
impl PasswordHasher for BcryptPasswordHasher {
    async fn hash(&self, password: &str) -> AppResult<String> {
        let password = password.to_owned();
        tokio::task::spawn_blocking(move || bcrypt::hash(password, COST))
            .await
            .map_err(AppError::internal)?
            .map_err(|_| AppError::Internal("Password hashing failed".into()))
    }

    async fn verify(&self, password: &str, hash: &str) -> bool {
        let (password, hash) = (password.to_owned(), hash.to_owned());
        tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash).unwrap_or(false))
            .await
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn verifies_spring_2y_hash_from_seed_migration() {
        // The admin seed in 0001_create_users_table.sql ("admin").
        let seed = "$2y$10$Dm42cN7wQvU5L9fhOP4E5uOyogeKOvq4i7nuhhHcdPBW8ZD5Fd/0q";
        let h = BcryptPasswordHasher;
        assert!(h.verify("admin", seed).await);
        assert!(!h.verify("nope", seed).await);
        let new = h.hash("s3cret-pass").await.unwrap();
        assert!(h.verify("s3cret-pass", &new).await);
    }
}
