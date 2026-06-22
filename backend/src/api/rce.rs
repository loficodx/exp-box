use axum::{Json, extract::State};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{api::error::ApiError, state::AppState};

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
) -> Result<Json<serde_json::Value>, ApiError> {
    let resp = state
        .http
        .post("http://room-rce:9000/exec")
        .json(&*body)
        .send()
        .await
        .map_err(|err| {
            ApiError::bad_gateway(
                "room_unavailable",
                format!("failed to call room-rce: {err}"),
            )
        })?;

    let status = resp.status();
    
    let text = resp.text().await.map_err(|err| {
        ApiError::bad_gateway(
            "room_response_read_failed",
            format!("failed to read room-rce response body: {err}"),
        )
    })?;

    if !status.is_success() {
        return Err(ApiError::bad_gateway(
            "room_bad_status",
            format!("room-rce returned status {status}: {text}"),
        ));
    }

    let json: serde_json::Value = serde_json::from_str(&text).map_err(|err| {
        ApiError::bad_gateway(
            "room_invalid_json",
            format!("failed to parse room-rce response as JSON: {err}; body: {text}"),
        )
    })?;

    Ok(Json(json))
}

pub async fn submit(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<SubmitRequest>,
) -> Result<(CookieJar, Json<SubmitResponse>), ApiError> {
    let hash = hex::encode(Sha256::digest(body.flag.trim().as_bytes()));

    let flag_hash: Option<String> =
        sqlx::query_scalar("SELECT flag_hash FROM rooms WHERE slug = 'rce'")
            .fetch_optional(&state.pool)
            .await
            .map_err(|err| {
                ApiError::internal(
                    "flag_hash_query_failed",
                    format!("failed to load RCE flag hash: {err}"),
                )
            })?;

    let correct = flag_hash.is_some_and(|h| h == hash);

    let sid = jar
        .get("sid")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    if correct {
        sqlx::query(
            "INSERT INTO progress (session_id, room_id, solved_at)
             SELECT ?1, id, CURRENT_TIMESTAMP FROM rooms WHERE slug = 'rce'
             ON CONFLICT(session_id, room_id) DO UPDATE SET solved_at = CURRENT_TIMESTAMP",
        )
            .bind(&sid)
            .execute(&state.pool)
            .await
            .map_err(|err| {
                ApiError::internal(
                    "progress_update_failed",
                    format!("failed to update RCE progress: {err}"),
                )
            })?;
    }

    let jar = jar.add(Cookie::new("sid", sid));

    Ok((jar, Json(SubmitResponse { correct })))
}