use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, state::AppState};

const SESSION_COOKIE: &str = "lab_session";
const SESSION_TTL_DAYS: i64 = 7;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct VerifySessionRequest {
    session_id: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    message: &'static str,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: &'static str,
    username: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    user_id: String,
    username: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<MessageResponse>, ApiError> {
    let username = body.username.trim();

    if username.is_empty() {
        return Err(ApiError::bad_request(
            "validation_error",
            "username is required",
        ));
    }
    if body.password.len() < 8 {
        return Err(ApiError::bad_request(
            "validation_error",
            "password must be at least 8 characters",
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| ApiError::internal("hash_failed", e.to_string()))?
        .to_string();

    let user_id = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES (?1, ?2, ?3)")
        .bind(&user_id)
        .bind(username)
        .bind(&password_hash)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                ApiError::conflict("username_taken", "username is already taken")
            } else {
                ApiError::internal("db_error", format!("failed to create user: {e}"))
            }
        })?;

    Ok(Json(MessageResponse {
        message: "registered",
    }))
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: String,
        username: String,
        password_hash: String,
    }

    let user: Option<UserRow> =
        sqlx::query_as("SELECT id, username, password_hash FROM users WHERE username = ?1")
            .bind(body.username.trim())
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| ApiError::internal("db_error", format!("failed to query user: {e}")))?;

    let user = user.ok_or_else(|| {
        ApiError::unauthorized("invalid_credentials", "invalid username or password")
    })?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| ApiError::internal("hash_parse_failed", e.to_string()))?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| {
            ApiError::unauthorized("invalid_credentials", "invalid username or password")
        })?;

    let session_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO sessions (id, user_id, expires_at)
         VALUES (?1, ?2, datetime('now', ?3))",
    )
    .bind(&session_id)
    .bind(&user.id)
    .bind(format!("+{SESSION_TTL_DAYS} days"))
    .execute(&state.pool)
    .await
    .map_err(|e| {
        ApiError::internal("db_error", format!("failed to create session: {e}"))
    })?;

    let cookie = Cookie::build((SESSION_COOKIE, session_id))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .build();

    Ok((
        jar.add(cookie),
        Json(LoginResponse {
            message: "logged in",
            username: user.username,
        }),
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<MessageResponse>), ApiError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        sqlx::query("DELETE FROM sessions WHERE id = ?1")
            .bind(cookie.value())
            .execute(&state.pool)
            .await
            .map_err(|e| {
                ApiError::internal("db_error", format!("failed to delete session: {e}"))
            })?;
    }

    let removal = Cookie::build((SESSION_COOKIE, "")).path("/").build();

    Ok((
        jar.remove(removal),
        Json(MessageResponse {
            message: "logged out",
        }),
    ))
}

pub async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<UserResponse>, ApiError> {
    let session_id = jar
        .get(SESSION_COOKIE)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::unauthorized("no_session", "not authenticated"))?;

    resolve_session(&state, &session_id).await.map(Json)
}

pub async fn verify_session(
    State(state): State<AppState>,
    Json(body): Json<VerifySessionRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    resolve_session(&state, &body.session_id).await.map(Json)
}

async fn resolve_session(state: &AppState, session_id: &str) -> Result<UserResponse, ApiError> {
    #[derive(sqlx::FromRow)]
    struct SessionUser {
        user_id: String,
        username: String,
    }

    let row: Option<SessionUser> = sqlx::query_as(
        "SELECT u.id AS user_id, u.username
         FROM sessions s
         JOIN users u ON u.id = s.user_id
         WHERE s.id = ?1 AND s.expires_at > CURRENT_TIMESTAMP",
    )
    .bind(session_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| ApiError::internal("db_error", format!("failed to query session: {e}")))?;

    row.map(|r| UserResponse {
        user_id: r.user_id,
        username: r.username,
    })
    .ok_or_else(|| ApiError::unauthorized("invalid_session", "session is invalid or expired"))
}
