use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod auth;
pub mod error;
pub mod health;
pub mod proxy;
pub mod rce;
pub mod rooms;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/rooms", get(rooms::list_rooms))
        .route("/rooms/rce/exec", post(rce::exec))
        .route("/rooms/rce/submit", post(rce::submit))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me))
}
