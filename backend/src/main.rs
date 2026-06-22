use axum::{Json, Router, routing::{get, post}};
use serde::Serialize;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

mod proxy;
mod rooms;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub http: reqwest::Client,
}

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

    let state = AppState {
        pool,
        http: reqwest::Client::new(),
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/rooms", get(rooms::list_rooms))
        .route("/api/rooms/rce/exec", post(proxy::exec))
        .route("/api/rooms/rce/submit", post(proxy::submit))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}
