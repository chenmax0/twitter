use axum::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::domain::{
    authentication_error::AuthenticationError, user::User, user_repository::UserRepository,
};

pub struct PgUserRepository {
    repository: PgPool,
}

impl PgUserRepository {
    pub fn new(repository: PgPool) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_by_email(&self, email: String) -> Result<Option<User>, AuthenticationError> {
        sqlx::query_as!(
            User,
            "SELECT id, email, password, pseudo FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.repository)
        .await
        .map_err(|error: Error| AuthenticationError::DatabaseError(error.to_string()))
    }

    async fn register(&self, email: String, password: String) -> Result<Uuid, AuthenticationError> {
        let id = Uuid::now_v7();

        sqlx::query!(
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3)",
            id,
            email,
            password
        )
        .execute(&self.repository)
        .await
        .map_err(|error: Error| AuthenticationError::DatabaseError(error.to_string()))?;

        Ok(id)
    }

    async fn login(&self, email: String, password: String) -> Result<Uuid, AuthenticationError> {
        let row = sqlx::query!(
            "SELECT id FROM users WHERE email = $1 AND password = $2",
            email,
            password,
        )
        .fetch_one(&self.repository)
        .await
        .map_err(|error: Error| AuthenticationError::DatabaseError(error.to_string()))?;

        Ok(row.id)
    }
}
