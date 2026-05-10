use api::infrastructure::config::routes;
use database::connection::Database;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database = Database::new()
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("../../migrations")
        .run(&database.pool())
        .await
        .expect("Failed to run migrations");

    let app = routes::create(database.pool());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
