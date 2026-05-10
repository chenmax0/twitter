use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserSession {
    pub user_id: Uuid,
    pub token: String,
    pub refresh: String,
    pub token_expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl UserSession {
    pub fn is_valid(&self) -> bool {
        self.revoked_at.is_none() && self.token_expires_at > Utc::now()
    }
}
