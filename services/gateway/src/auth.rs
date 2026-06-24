use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

/// Authenticated platform user, resolved from the `lab_session` cookie.
///
/// Use as an Axum extractor on any handler that requires authentication:
///
/// ```rust
/// async fn my_handler(user: AuthUser, ...) { ... }
/// ```
///
/// Returns 401 if the cookie is absent or the session is invalid/expired.
/// Returns 502 if auth-service is unreachable.
pub struct AuthUser {
    pub user_id: String,
    pub username: String,
}

#[derive(Serialize)]
struct VerifyRequest<'a> {
    session_id: &'a str,
}

#[derive(Deserialize)]
struct VerifyResponse {
    user_id: String,
    username: String,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: &'static str,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session_id = extract_session_cookie(&parts.headers)
            .ok_or_else(|| reject(StatusCode::UNAUTHORIZED, "unauthorized", "not authenticated"))?;

        let resp = state
            .http
            .post(format!("{}/sessions/verify", state.auth_service_url))
            .json(&VerifyRequest {
                session_id: &session_id,
            })
            .send()
            .await
            .map_err(|_| {
                reject(
                    StatusCode::BAD_GATEWAY,
                    "auth_service_unavailable",
                    "authentication service is temporarily unavailable",
                )
            })?;

        if !resp.status().is_success() {
            return Err(reject(
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "session is invalid or expired",
            ));
        }

        let body: VerifyResponse = resp.json().await.map_err(|_| {
            reject(
                StatusCode::BAD_GATEWAY,
                "auth_service_unavailable",
                "authentication service is temporarily unavailable",
            )
        })?;

        Ok(AuthUser {
            user_id: body.user_id,
            username: body.username,
        })
    }
}

/// Try to resolve a session without requiring authentication.
/// Returns `None` if the cookie is absent, the session is invalid, or auth-service is down.
/// Use this in handlers where authentication is optional (e.g. room listing).
pub async fn try_authenticate(
    state: &AppState,
    headers: &axum::http::HeaderMap,
) -> Option<AuthUser> {
    let session_id = extract_session_cookie(headers)?;

    let resp = state
        .http
        .post(format!("{}/sessions/verify", state.auth_service_url))
        .json(&VerifyRequest {
            session_id: &session_id,
        })
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    resp.json::<VerifyResponse>()
        .await
        .ok()
        .map(|body| AuthUser {
            user_id: body.user_id,
            username: body.username,
        })
}

fn extract_session_cookie(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .find_map(|part| part.trim().strip_prefix("lab_session="))
        })
        .map(str::to_string)
}

fn reject(status: StatusCode, error: &'static str, message: &'static str) -> Response {
    (status, Json(ErrorBody { error, message })).into_response()
}
