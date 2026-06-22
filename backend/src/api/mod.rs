use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod error;
pub mod health;
pub mod rce;
pub mod rooms;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/rooms", get(rooms::list_rooms))
        .route("/rooms/rce/exec", post(rce::exec))
        .route("/rooms/rce/submit", post(rce::submit))
}
