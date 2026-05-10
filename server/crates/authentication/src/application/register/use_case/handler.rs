use crate::{
    application::register::dto::payload::RegisterPayload,
    domain::{
        authentication_error::AuthenticationError, user_repository::UserRepository,
        user_session::UserSession,
    },
    infrastructure::service::{password_service, token_service::generate_user_session},
};
use std::sync::Arc;

pub struct RegisterHandler<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> RegisterHandler<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        payload: RegisterPayload,
    ) -> Result<UserSession, AuthenticationError> {
        let user_exists = self.repository.get_by_email(payload.email.clone()).await?;

        if user_exists.is_some() {
            return Err(AuthenticationError::UserAlreadyExists);
        }

        let hashed_password = password_service::hash(payload.password)?;

        let id = self
            .repository
            .register(payload.email, hashed_password)
            .await?;
        let user_session = generate_user_session(id).await?;

        Ok(user_session)
    }
}
