use axum::async_trait;
use uuid::Uuid;

use crate::domain::{authentication_error::AuthenticationError, user::User};

#[async_trait]
pub trait UserRepository {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, AuthenticationError>;
    async fn login(&self, email: String, password: String) -> Result<Uuid, AuthenticationError>;
    async fn register(&self, email: String, password: String) -> Result<Uuid, AuthenticationError>;
}
