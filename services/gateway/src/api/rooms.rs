use axum::{Json, extract::State, http::HeaderMap};
use serde::Serialize;

use crate::{api::error::ApiError, auth, state::AppState};

#[derive(sqlx::FromRow)]
struct RoomRow {
    slug: String,
    title: String,
    category: String,
    difficulty: String,
    position: i32,
    description: String,
    solved: bool,
}

#[derive(Serialize)]
pub struct RoomResponse {
    slug: String,
    title: String,
    category: String,
    difficulty: String,
    position: i32,
    description: String,
    solved: bool,
}

impl From<RoomRow> for RoomResponse {
    fn from(row: RoomRow) -> Self {
        Self {
            slug: row.slug,
            title: row.title,
            category: row.category,
            difficulty: row.difficulty,
            position: row.position,
            description: row.description,
            solved: row.solved,
        }
    }
}

pub async fn list_rooms(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<RoomResponse>>, ApiError> {
    // Authentication is optional: unauthenticated users see rooms but solved is always false.
    let user_id = auth::try_authenticate(&state, &headers)
        .await
        .map(|u| u.user_id)
        .unwrap_or_default();

    let rows: Vec<RoomRow> = sqlx::query_as(
        "SELECT
            r.slug,
            r.title,
            r.category,
            r.difficulty,
            r.position,
            r.description,
            COALESCE(p.solved_at IS NOT NULL, FALSE) AS solved
         FROM rooms r
         LEFT JOIN progress p
            ON p.room_id = r.id
            AND p.user_id = ?1
         ORDER BY r.position",
    )
    .bind(&user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|err| {
        ApiError::internal("rooms_query_failed", format!("failed to load rooms: {err}"))
    })?;

    let rooms = rows.into_iter().map(RoomResponse::from).collect();

    Ok(Json(rooms))
}
