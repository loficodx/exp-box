use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{api::error::ApiError, auth::AuthUser, state::AppState};

const RCE_SLUG: &str = "rce";

#[derive(Deserialize)]
pub struct SubmitRequest {
    flag: String,
}

#[derive(Serialize)]
pub struct SubmitResponse {
    correct: bool,
}

#[derive(sqlx::FromRow)]
struct RoomFlag {
    id: i64,
    flag_hash: String,
}

// INTENTIONALLY VULNERABLE — training target: forwards cmd verbatim, no inspection
pub async fn exec(
    State(state): State<AppState>,
    body: Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let room = state.rooms.get(RCE_SLUG).ok_or_else(|| {
        ApiError::internal(
            "room_target_missing",
            format!("room target is not configured for slug {RCE_SLUG}"),
        )
    })?;

    let resp = state
        .http
        .post(room.action_url("exec"))
        .json(&*body)
        .send()
        .await
        .map_err(|err| {
            ApiError::bad_gateway(
                "room_unavailable",
                format!("failed to call {} room target: {err}", room.slug),
            )
        })?;

    let status = resp.status();

    let text = resp.text().await.map_err(|err| {
        ApiError::bad_gateway(
            "room_response_read_failed",
            format!("failed to read {} room response body: {err}", room.slug),
        )
    })?;

    if !status.is_success() {
        return Err(ApiError::bad_gateway(
            "room_bad_status",
            format!("{} room returned status {status}: {text}", room.slug),
        ));
    }

    let json: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        ApiError::bad_gateway(
            "room_invalid_json",
            format!(
                "failed to parse {} room response as JSON: {err}; body: {text}",
                room.slug
            ),
        )
    })?;

    Ok(Json(json))
}

pub async fn submit(
    State(state): State<AppState>,
    user: AuthUser,
    Json(body): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, ApiError> {
    submit_for_slug(&state, &user, RCE_SLUG, body).await
}

pub async fn submit_by_slug(
    State(state): State<AppState>,
    user: AuthUser,
    Path(slug): Path<String>,
    Json(body): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, ApiError> {
    submit_for_slug(&state, &user, &slug, body).await
}

async fn submit_for_slug(
    state: &AppState,
    user: &AuthUser,
    slug: &str,
    body: SubmitRequest,
) -> Result<Json<SubmitResponse>, ApiError> {
    let hash = hex::encode(Sha256::digest(body.flag.trim().as_bytes()));

    let room: Option<RoomFlag> = sqlx::query_as("SELECT id, flag_hash FROM rooms WHERE slug = ?1")
        .bind(slug)
        .fetch_optional(&state.pool)
        .await
        .map_err(|err| {
            ApiError::internal(
                "flag_hash_query_failed",
                format!("failed to load {slug} flag hash: {err}"),
            )
        })?;

    let correct = room.as_ref().is_some_and(|room| room.flag_hash == hash);

    if let Some(room) = room.filter(|_| correct) {
        sqlx::query(
            "INSERT INTO progress (user_id, room_id, solved_at)
             VALUES (?1, ?2, CURRENT_TIMESTAMP)
             ON CONFLICT(user_id, room_id) DO UPDATE SET solved_at = CURRENT_TIMESTAMP",
        )
        .bind(&user.user_id)
        .bind(room.id)
        .execute(&state.pool)
        .await
        .map_err(|err| {
            ApiError::internal(
                "progress_update_failed",
                format!("failed to update {slug} progress: {err}"),
            )
        })?;
    }

    Ok(Json(SubmitResponse { correct }))
}
