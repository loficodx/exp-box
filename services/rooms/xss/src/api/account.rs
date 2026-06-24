use axum::{Json, extract::State, http::HeaderMap};
use serde::{Deserialize, Serialize};

use crate::{api::comments::header_str, error::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct RoomAccount {
    pub user_id: String,
    pub display_password_value: String,
    pub updated_at: String,
}

// No CSRF token is checked — this is intentional for the CSRF training room.
pub async fn change_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<Json<RoomAccount>, ApiError> {
    let user_id = header_str(&headers, "x-lab-user-id")
        .ok_or_else(|| ApiError::unauthorized("missing_identity", "X-Lab-User-Id header is required"))?;

    if body.new_password.trim().is_empty() {
        return Err(ApiError::bad_request(
            "empty_password",
            "new_password must not be empty",
        ));
    }

    sqlx::query(
        "INSERT INTO room_accounts (user_id, display_password_value, updated_at)
         VALUES (?1, ?2, CURRENT_TIMESTAMP)
         ON CONFLICT(user_id) DO UPDATE
           SET display_password_value = excluded.display_password_value,
               updated_at = CURRENT_TIMESTAMP",
    )
    .bind(&user_id)
    .bind(&body.new_password)
    .execute(&state.pool)
    .await
    .map_err(|e| ApiError::internal("account_upsert_failed", e.to_string()))?;

    let account: RoomAccount = sqlx::query_as(
        "SELECT user_id, display_password_value, updated_at FROM room_accounts WHERE user_id = ?1",
    )
    .bind(&user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| ApiError::internal("account_fetch_failed", e.to_string()))?;

    Ok(Json(account))
}
