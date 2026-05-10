use std::sync::Arc;

use crate::infrastructure::repository::user_repository::PgUserRepository;

#[derive(Clone)]
pub struct AuthenticationState {
    pub user_repository: Arc<PgUserRepository>,
}

impl AuthenticationState {
    pub fn new(user_repository: Arc<PgUserRepository>) -> Self {
        Self { user_repository }
    }
}
