use axum::{Router, routing::get};

use crate::state::AppState;

pub mod auth;
pub mod health;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health::health))
}
