use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::domain::authentication_error::AuthenticationError;

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // --- Token
            Self::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Veuillez vous reconnecter.".to_string(),
            ),

            // --- User
            Self::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "L'utilisateur existe déjà".to_string(),
            ),
            Self::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "L'e-mail ou le mot de passe est erroné.".to_string(),
            ),

            // --- General
            Self::DatabaseError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
