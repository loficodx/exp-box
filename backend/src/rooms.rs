use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::AppState;

#[derive(sqlx::FromRow)]
struct RoomRow {
    slug: String,
    title: String,
    category: String,
    difficulty: String,
    position: i32,
    description: String,
    solved: Option<bool>,
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

pub async fn list_rooms(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Json<Vec<RoomResponse>> {
    let sid = jar.get("sid").map(|c| c.value().to_string()).unwrap_or_default();

    let rows: Vec<RoomRow> = sqlx::query_as(
        "SELECT r.slug, r.title, r.category, r.difficulty, r.position, r.description,
         (p.solved_at IS NOT NULL) AS solved
         FROM rooms r
         LEFT JOIN progress p ON p.room_id = r.id AND p.session_id = $1
         ORDER BY r.position",
    )
    .bind(&sid)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    Json(
        rows.into_iter()
            .map(|r| RoomResponse {
                slug: r.slug,
                title: r.title,
                category: r.category,
                difficulty: r.difficulty,
                position: r.position,
                description: r.description,
                solved: r.solved.unwrap_or(false),
            })
            .collect(),
    )
}
