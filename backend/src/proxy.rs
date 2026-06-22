use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::AppState;

#[derive(Deserialize)]
pub struct SubmitRequest {
    flag: String,
}

#[derive(Serialize)]
pub struct SubmitResponse {
    correct: bool,
}

// INTENTIONALLY VULNERABLE — training target: forwards cmd verbatim, no inspection
pub async fn exec(
    State(state): State<AppState>,
    body: Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let resp = state
        .http
        .post("http://room-rce:9000/exec")
        .json(&*body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let json: serde_json::Value = resp.json().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(json))
}

pub async fn submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<SubmitRequest>,
) -> (CookieJar, Json<SubmitResponse>) {
    let hash = hex::encode(Sha256::digest(body.flag.trim().as_bytes()));

    let flag_hash: Option<String> =
        sqlx::query_scalar("SELECT flag_hash FROM rooms WHERE slug = 'rce'")
            .fetch_optional(&state.pool)
            .await
            .unwrap_or(None);

    let correct = flag_hash.map_or(false, |h| h == hash);

    let sid = jar
        .get("sid")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    if correct {
        sqlx::query(
            "INSERT INTO progress (session_id, room_id, solved_at)
             SELECT $1, id, now() FROM rooms WHERE slug = 'rce'
             ON CONFLICT (session_id, room_id) DO UPDATE SET solved_at = now()",
        )
        .bind(&sid)
        .execute(&state.pool)
        .await
        .ok();
    }

    let jar = jar.add(Cookie::new("sid", sid));
    (jar, Json(SubmitResponse { correct }))
}
