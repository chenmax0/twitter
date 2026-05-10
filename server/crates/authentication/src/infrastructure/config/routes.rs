use std::sync::Arc;

use crate::infrastructure::{
    config::state::AuthenticationState,
    controller::authentication_controller::{login, register},
    repository::user_repository::PgUserRepository,
};

use axum::{Router, routing::post};
use sqlx::PgPool;

pub fn public_routes(pool: PgPool) -> Router {
    let state = AuthenticationState {
        user_repository: Arc::new(PgUserRepository::new(pool)),
    };

    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(state)
}
