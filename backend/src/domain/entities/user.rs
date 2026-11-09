use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::User => "USER",
            Role::Admin => "ADMIN",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "USER" => Some(Role::User),
            "ADMIN" => Some(Role::Admin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub failed_login_attempts: i32,
    pub locked_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn is_locked(&self, now: NaiveDateTime) -> bool {
        self.locked_until.is_some_and(|until| now < until)
    }

    /// Failed attempts that still count towards a lockout
    pub fn effective_failed_attempts(&self, now: NaiveDateTime) -> i32 {
        match self.locked_until {
            Some(until) if now >= until => 0,
            _ => self.failed_login_attempts,
        }
    }
}

/// Who is performing a request; used for ownership checks.
#[derive(Debug, Clone, Copy)]
pub struct Requester {
    pub id: Uuid,
    pub is_admin: bool,
}

impl Requester {
    pub fn can_manage(&self, user_id: Uuid) -> bool {
        self.is_admin || self.id == user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn user(attempts: i32, locked_until: Option<NaiveDateTime>) -> User {
        let now = Utc::now().naive_utc();
        User {
            id: Uuid::nil(),
            name: "a".into(),
            email: "a@b.c".into(),
            password_hash: String::new(),
            role: Role::User,
            failed_login_attempts: attempts,
            locked_until,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn lock_expiry_resets_attempt_counter() {
        let now = Utc::now().naive_utc();
        let locked = user(5, Some(now + Duration::minutes(5)));
        assert!(locked.is_locked(now));
        assert_eq!(locked.effective_failed_attempts(now), 5);

        let expired = user(5, Some(now - Duration::minutes(5)));
        assert!(!expired.is_locked(now));
        assert_eq!(expired.effective_failed_attempts(now), 0);
    }

    #[test]
    fn role_round_trip() {
        for r in [Role::User, Role::Admin] {
            assert_eq!(Role::parse(r.as_str()), Some(r));
        }
        assert_eq!(Role::parse("admin"), None);
    }
}
