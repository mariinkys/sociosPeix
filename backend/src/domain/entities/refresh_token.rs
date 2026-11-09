use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub token: String,
    pub user_email: String,
    pub expires_at: NaiveDateTime,
    pub revoked: bool,
    pub created_at: NaiveDateTime,
}

impl RefreshToken {
    pub fn is_expired(&self, now: NaiveDateTime) -> bool {
        now > self.expires_at
    }
}
