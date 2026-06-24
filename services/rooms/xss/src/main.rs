use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    message: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        message: "room-xss ready",
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("xss placeholder listening on http://0.0.0.0:9000");
    axum::serve(listener, app).await.unwrap();
}
