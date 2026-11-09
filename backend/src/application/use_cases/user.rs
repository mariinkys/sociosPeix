use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    domain::pagination::{Page, PageRequest, SortDirection},
    entities::user::{Requester, Role, User},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserSortField {
    Name,
    Email,
    Role,
    #[default]
    CreatedAt,
}

impl UserSortField {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "name" => Self::Name,
            "email" => Self::Email,
            "role" => Self::Role,
            "createdAt" => Self::CreatedAt,
            _ => return None,
        })
    }
}

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn list_users(
        &self,
        search: &str,
        sort: (UserSortField, SortDirection),
        page: PageRequest,
    ) -> AppResult<Page<User>>;
    /// Fails with [`AppError::Conflict`] if the email is taken.
    async fn create_user(&self, name: &str, email: &str, password_hash: &str) -> AppResult<User>;
    /// Fails with [`AppError::Conflict`] if the email is taken.
    async fn update_user_details(&self, id: Uuid, name: &str, email: &str) -> AppResult<User>;
    async fn update_user_role(&self, id: Uuid, role: Role) -> AppResult<User>;
    async fn update_user_password(&self, id: Uuid, password_hash: &str) -> AppResult<()>;
    async fn reset_failed_logins(&self, id: Uuid) -> AppResult<()>;
    async fn record_failed_login(
        &self,
        id: Uuid,
        attempts: i32,
        locked_until: Option<NaiveDateTime>,
    ) -> AppResult<()>;
    async fn delete_user(&self, id: Uuid) -> AppResult<()>;
}

/// Password hashing is CPU-bound; implementations must not block the async runtime.
#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> AppResult<String>;
    async fn verify(&self, password: &str, hash: &str) -> bool;
}

pub struct UserUseCases {
    users: Arc<dyn UserPersistence>,
    hasher: Arc<dyn PasswordHasher>,
}

fn not_found(id: Uuid) -> AppError {
    AppError::NotFound(format!("User not found with id: {id}"))
}

impl UserUseCases {
    pub fn new(users: Arc<dyn UserPersistence>, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self { users, hasher }
    }

    #[instrument(skip(self, password))]
    pub async fn create_user(&self, name: &str, email: &str, password: &str) -> AppResult<User> {
        if self.users.find_user_by_email(email).await?.is_some() {
            return Err(AppError::Conflict(format!("Email already in use: {email}")));
        }
        let hash = self.hasher.hash(password).await?;
        let user = self.users.create_user(name, email, &hash).await?;
        info!(user_id = %user.id, "user created");
        Ok(user)
    }

    pub async fn get_user(&self, id: Uuid, requester: Requester) -> AppResult<User> {
        let user = self
            .users
            .find_user_by_id(id)
            .await?
            .ok_or_else(|| not_found(id))?;
        if !requester.can_manage(user.id) {
            return Err(AppError::Forbidden(
                "You can only access your own account".into(),
            ));
        }
        Ok(user)
    }

    pub async fn list_users(
        &self,
        search: &str,
        sort: (UserSortField, SortDirection),
        page: PageRequest,
    ) -> AppResult<Page<User>> {
        self.users.list_users(search.trim(), sort, page).await
    }

    #[instrument(skip(self))]
    pub async fn update_user(
        &self,
        id: Uuid,
        name: &str,
        email: &str,
        requester: Requester,
    ) -> AppResult<User> {
        let user = self
            .users
            .find_user_by_id(id)
            .await?
            .ok_or_else(|| not_found(id))?;
        if !requester.can_manage(user.id) {
            return Err(AppError::Forbidden(
                "You can only update your own account".into(),
            ));
        }
        self.users.update_user_details(id, name, email).await
    }

    #[instrument(skip(self))]
    pub async fn update_role(&self, id: Uuid, role: Role) -> AppResult<User> {
        if self.users.find_user_by_id(id).await?.is_none() {
            return Err(not_found(id));
        }
        self.users.update_user_role(id, role).await
    }

    /// Admins may reset anyone's password without knowing the current one;
    /// everybody else must prove they know their own.
    #[instrument(skip(self, current_password, new_password))]
    pub async fn change_password(
        &self,
        id: Uuid,
        current_password: Option<&str>,
        new_password: &str,
        requester: Requester,
    ) -> AppResult<()> {
        let user = self
            .users
            .find_user_by_id(id)
            .await?
            .ok_or_else(|| not_found(id))?;
        if !requester.is_admin {
            if requester.id != user.id {
                return Err(AppError::Forbidden(
                    "You can only change your own password".into(),
                ));
            }
            let current = current_password.filter(|p| !p.trim().is_empty());
            let ok = match current {
                Some(p) => self.hasher.verify(p, &user.password_hash).await,
                None => false,
            };
            if !ok {
                return Err(AppError::Unauthorized(
                    "Current password is incorrect".into(),
                ));
            }
        }
        let hash = self.hasher.hash(new_password).await?;
        self.users.update_user_password(id, &hash).await
    }

    #[instrument(skip(self))]
    pub async fn delete_user(&self, id: Uuid, requester: Requester) -> AppResult<()> {
        let user = self
            .users
            .find_user_by_id(id)
            .await?
            .ok_or_else(|| not_found(id))?;
        if !requester.can_manage(user.id) {
            return Err(AppError::Forbidden(
                "You can only delete your own account".into(),
            ));
        }
        self.users.delete_user(id).await
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use chrono::Utc;
    use std::sync::Mutex;

    /// In-memory `UserPersistence` for use-case tests.
    #[derive(Default)]
    pub struct InMemoryUsers {
        pub users: Mutex<Vec<User>>,
    }

    impl InMemoryUsers {
        pub fn with(users: Vec<User>) -> Self {
            Self {
                users: Mutex::new(users),
            }
        }

        fn update(&self, id: Uuid, f: impl FnOnce(&mut User)) -> AppResult<User> {
            let mut users = self.users.lock().unwrap();
            let user = users
                .iter_mut()
                .find(|u| u.id == id)
                .ok_or_else(|| not_found(id))?;
            f(user);
            Ok(user.clone())
        }
    }

    pub fn user(email: &str, role: Role, password_hash: &str) -> User {
        let now = Utc::now().naive_utc();
        User {
            id: Uuid::new_v4(),
            name: "Test".into(),
            email: email.into(),
            password_hash: password_hash.into(),
            role,
            failed_login_attempts: 0,
            locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[async_trait]
    impl UserPersistence for InMemoryUsers {
        async fn find_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.id == id)
                .cloned())
        }
        async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.email == email)
                .cloned())
        }
        async fn list_users(
            &self,
            _: &str,
            _: (UserSortField, SortDirection),
            page: PageRequest,
        ) -> AppResult<Page<User>> {
            let items = self.users.lock().unwrap().clone();
            let total = items.len() as i64;
            Ok(Page {
                items,
                request: page,
                total,
            })
        }
        async fn create_user(&self, name: &str, email: &str, hash: &str) -> AppResult<User> {
            let mut u = user(email, Role::User, hash);
            u.name = name.into();
            self.users.lock().unwrap().push(u.clone());
            Ok(u)
        }
        async fn update_user_details(&self, id: Uuid, name: &str, email: &str) -> AppResult<User> {
            self.update(id, |u| {
                u.name = name.into();
                u.email = email.into();
            })
        }
        async fn update_user_role(&self, id: Uuid, role: Role) -> AppResult<User> {
            self.update(id, |u| u.role = role)
        }
        async fn update_user_password(&self, id: Uuid, hash: &str) -> AppResult<()> {
            self.update(id, |u| u.password_hash = hash.into())
                .map(|_| ())
        }
        async fn reset_failed_logins(&self, id: Uuid) -> AppResult<()> {
            self.update(id, |u| {
                u.failed_login_attempts = 0;
                u.locked_until = None;
            })
            .map(|_| ())
        }
        async fn record_failed_login(
            &self,
            id: Uuid,
            attempts: i32,
            locked_until: Option<NaiveDateTime>,
        ) -> AppResult<()> {
            self.update(id, |u| {
                u.failed_login_attempts = attempts;
                u.locked_until = locked_until;
            })
            .map(|_| ())
        }
        async fn delete_user(&self, id: Uuid) -> AppResult<()> {
            self.users.lock().unwrap().retain(|u| u.id != id);
            Ok(())
        }
    }

    /// "Hashes" by prefixing, so tests stay fast and deterministic.
    pub struct PlainHasher;

    #[async_trait]
    impl PasswordHasher for PlainHasher {
        async fn hash(&self, password: &str) -> AppResult<String> {
            Ok(format!("hashed:{password}"))
        }
        async fn verify(&self, password: &str, hash: &str) -> bool {
            hash == format!("hashed:{password}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;

    fn setup() -> (UserUseCases, Arc<InMemoryUsers>, User, User) {
        let admin = user("admin@x.com", Role::Admin, "hashed:admin");
        let bob = user("bob@x.com", Role::User, "hashed:secret123");
        let users = Arc::new(InMemoryUsers::with(vec![admin.clone(), bob.clone()]));
        (
            UserUseCases::new(users.clone(), Arc::new(PlainHasher)),
            users,
            admin,
            bob,
        )
    }

    fn req(u: &User) -> Requester {
        Requester {
            id: u.id,
            is_admin: u.is_admin(),
        }
    }

    #[tokio::test]
    async fn duplicate_email_is_a_conflict() {
        let (uc, _, _, _) = setup();
        let err = uc
            .create_user("Bob", "bob@x.com", "password1")
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::Conflict(_)));
    }

    #[tokio::test]
    async fn users_can_only_see_themselves() {
        let (uc, _, admin, bob) = setup();
        assert!(uc.get_user(bob.id, req(&bob)).await.is_ok());
        assert!(matches!(
            uc.get_user(admin.id, req(&bob)).await,
            Err(AppError::Forbidden(_))
        ));
        assert!(uc.get_user(bob.id, req(&admin)).await.is_ok());
    }

    #[tokio::test]
    async fn non_admin_needs_current_password() {
        let (uc, users, admin, bob) = setup();
        let err = uc
            .change_password(bob.id, None, "newpass12", req(&bob))
            .await;
        assert!(matches!(err, Err(AppError::Unauthorized(_))));
        let err = uc
            .change_password(bob.id, Some("wrong"), "newpass12", req(&bob))
            .await;
        assert!(matches!(err, Err(AppError::Unauthorized(_))));
        uc.change_password(bob.id, Some("secret123"), "newpass12", req(&bob))
            .await
            .unwrap();
        // Admin doesn't need it.
        uc.change_password(bob.id, None, "adminset1", req(&admin))
            .await
            .unwrap();
        let stored = users.find_user_by_id(bob.id).await.unwrap().unwrap();
        assert_eq!(stored.password_hash, "hashed:adminset1");
    }
}
