use axum::Router;
use sqlx::PgPool;

pub fn create(pool: PgPool) -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .merge(authentication::infrastructure::config::routes::public_routes(pool.clone())),
    )
}
