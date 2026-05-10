use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    application::{
        login::{dto::payload::LoginPayload, use_case::handler::LoginHandler},
        register::{dto::payload::RegisterPayload, use_case::handler::RegisterHandler},
    },
    infrastructure::config::state::AuthenticationState,
};

pub async fn register(
    State(state): State<AuthenticationState>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let handler = RegisterHandler::new(state.user_repository.clone());

    match handler.execute(payload).await {
        Ok(user_session) => (StatusCode::CREATED, Json(json!(user_session))).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn login(
    State(state): State<AuthenticationState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let handler = LoginHandler::new(state.user_repository.clone());

    match handler.execute(payload).await {
        Ok(user_session) => (StatusCode::OK, Json(json!(user_session))).into_response(),
        Err(error) => error.into_response(),
    }
}
