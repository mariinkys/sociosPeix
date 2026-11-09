use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PasswordResetCode {
    pub id: Uuid,
    pub email: String,
    pub code_hash: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub attempts: i32,
    pub used: bool,
}

impl PasswordResetCode {
    pub const MAX_ATTEMPTS: i32 = 5;

    pub fn is_usable(&self, now: NaiveDateTime) -> bool {
        !self.used && now <= self.expires_at && self.attempts < Self::MAX_ATTEMPTS
    }
}
