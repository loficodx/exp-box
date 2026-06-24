use axum::{Router, routing::get};

use crate::state::AppState;

pub mod account;
pub mod comments;
pub mod health;
pub mod post;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/post", get(post::get_post))
}
