use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod auth;
pub mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/me", get(auth::me))
        .route("/sessions/verify", post(auth::verify_session))
}
