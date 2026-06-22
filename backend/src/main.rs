use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct HealthResponse {
    message: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { message: "exp-box ready" })
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("failed to connect to database");

    sqlx::migrate!("../migrations").run(&pool).await.expect("failed to run migrations");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}
