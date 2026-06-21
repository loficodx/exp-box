use axum::{Json, Router, routing::get};
use serde::Serialize;
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
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening on http://127.0.0.1:8000");
    axum::serve(listener, app).await.unwrap();
}
