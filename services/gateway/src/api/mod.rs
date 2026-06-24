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
pub mod xss;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/rooms", get(rooms::list_rooms))
        .route("/rooms/xss/post", get(xss::get_post))
        .route("/rooms/xss/comments", get(xss::get_comments))
        .route("/rooms/xss/comments", post(xss::post_comment))
        .route("/rooms/{slug}/actions/{action}", post(rce::action))
        .route("/rooms/{slug}/submit", post(rce::submit_by_slug))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me))
}
