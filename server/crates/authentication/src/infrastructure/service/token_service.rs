use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::{authentication_error::AuthenticationError, user_session::UserSession};

pub async fn generate_user_session(user_id: Uuid) -> Result<UserSession, AuthenticationError> {
    let now = Utc::now();

    let token = Uuid::now_v7().to_string();
    let refresh = Uuid::now_v7().to_string();

    let token_expires_at = now + Duration::minutes(15);
    let refresh_expires_at = now + Duration::weeks(1);

    Ok(UserSession {
        user_id,
        token,
        refresh,
        token_expires_at,
        refresh_expires_at,
        revoked_at: None,
    })
}
