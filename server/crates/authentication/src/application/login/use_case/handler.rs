use std::sync::Arc;

use crate::{
    application::login::dto::payload::LoginPayload,
    domain::{
        authentication_error::AuthenticationError, user_repository::UserRepository,
        user_session::UserSession,
    },
    infrastructure::service::{password_service, token_service::generate_user_session},
};

pub struct LoginHandler<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> LoginHandler<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, payload: LoginPayload) -> Result<UserSession, AuthenticationError> {
        let user = self
            .repository
            .get_by_email(payload.email)
            .await?
            .ok_or(AuthenticationError::InvalidCredentials)?;

        if !password_service::compare(payload.password, user.password)? {
            return Err(AuthenticationError::InvalidCredentials);
        }

        let user_session = generate_user_session(user.id).await?;
        Ok(user_session)
    }
}
