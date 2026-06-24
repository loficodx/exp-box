use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod account;
pub mod comments;
pub mod health;
pub mod post;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/post", get(post::get_post))
        .route("/comments", get(comments::list_comments))
        .route("/comments", post(comments::post_comment))
        .route("/change-password", post(account::change_password))
        .route("/reset", post(account::reset))
}
