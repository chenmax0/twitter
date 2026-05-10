use axum::async_trait;
use uuid::Uuid;

use crate::domain::{authentication_error::AuthenticationError, user_session::UserSession};

#[async_trait]
pub trait TokenRepository {
    async fn get(token: String) -> Result<Option<UserSession>, AuthenticationError>;
    async fn create(token: String, user_id: Uuid) -> Result<(), AuthenticationError>;
    async fn revoke(token: String) -> Result<(), AuthenticationError>;
}
