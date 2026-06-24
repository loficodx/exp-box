use axum::{
    Json,
    extract::State,
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, state::AppState};

#[derive(Serialize, sqlx::FromRow)]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub username: String,
    // Body is stored and returned as-is — intentionally not sanitized.
    pub body: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct PostCommentRequest {
    pub body: String,
}

pub async fn list_comments(
    State(state): State<AppState>,
) -> Result<Json<Vec<Comment>>, ApiError> {
    let comments: Vec<Comment> =
        sqlx::query_as("SELECT id, user_id, username, body, created_at FROM comments ORDER BY created_at ASC")
            .fetch_all(&state.pool)
            .await
            .map_err(|e| ApiError::internal("comments_query_failed", e.to_string()))?;

    Ok(Json(comments))
}

pub async fn post_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<PostCommentRequest>,
) -> Result<Json<Comment>, ApiError> {
    let user_id = header_str(&headers, "x-lab-user-id")
        .ok_or_else(|| ApiError::unauthorized("missing_identity", "X-Lab-User-Id header is required"))?;
    let username = header_str(&headers, "x-lab-username")
        .ok_or_else(|| ApiError::unauthorized("missing_identity", "X-Lab-Username header is required"))?;

    if body.body.trim().is_empty() {
        return Err(ApiError::bad_request("empty_body", "comment body must not be empty"));
    }

    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO comments (id, user_id, username, body) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(&id)
    .bind(&user_id)
    .bind(&username)
    .bind(&body.body)
    .execute(&state.pool)
    .await
    .map_err(|e| ApiError::internal("comment_insert_failed", e.to_string()))?;

    let comment: Comment =
        sqlx::query_as("SELECT id, user_id, username, body, created_at FROM comments WHERE id = ?1")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| ApiError::internal("comment_fetch_failed", e.to_string()))?;

    Ok(Json(comment))
}

fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
}
