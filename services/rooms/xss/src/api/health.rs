use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    message: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        message: "room-xss ready",
    })
}
