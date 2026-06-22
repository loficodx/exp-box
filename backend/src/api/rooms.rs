use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::{api::error::ApiError, state::AppState};

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
    jar: CookieJar,
) -> Result<Json<Vec<RoomResponse>>, ApiError> {
    let sid = jar
        .get("sid")
        .map(|cookie| cookie.value().to_string())
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
            AND p.session_id = ?1
         ORDER BY r.position",
    )
        .bind(&sid)
        .fetch_all(&state.pool)
        .await
        .map_err(|err| {
            ApiError::internal(
                "rooms_query_failed",
                format!("failed to load rooms: {err}"),
            )
        })?;

    let rooms = rows.into_iter().map(RoomResponse::from).collect();

    Ok(Json(rooms))
}