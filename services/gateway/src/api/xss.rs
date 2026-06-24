use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};

use super::proxy;
use crate::{api::error::ApiError, auth::AuthUser, state::AppState};

fn room_url(state: &AppState, slug: &str, action: &str) -> Result<String, ApiError> {
    let room = state.rooms.get(slug).ok_or_else(|| {
        ApiError::bad_request("unknown_room", format!("no room configured for slug {slug}"))
    })?;
    if !room.actions.contains(&action) {
        return Err(ApiError::bad_request(
            "unsupported_action",
            format!("room {slug} does not support action {action}"),
        ));
    }
    Ok(room.action_url(action))
}

/// POST to a room path with identity headers injected. Requires prior auth.
async fn proxy_authed_post(
    state: &AppState,
    user: &AuthUser,
    slug: &str,
    action: &str,
    headers: &HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let url = room_url(state, slug, action)?;

    let mut builder = state.http.post(&url);

    if let Some(ct) = headers.get(axum::http::header::CONTENT_TYPE) {
        builder = builder.header("Content-Type", ct.as_bytes());
    }

    let resp = builder
        .header("X-Lab-User-Id", &user.user_id)
        .header("X-Lab-Username", &user.username)
        .body(body)
        .send()
        .await
        .map_err(|e| {
            ApiError::bad_gateway(
                "room_unavailable",
                format!("room {slug} unreachable: {e}"),
            )
        })?;

    let status =
        StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let body_bytes = resp.bytes().await.map_err(|e| {
        ApiError::bad_gateway(
            "room_response_failed",
            format!("failed to read room {slug} response: {e}"),
        )
    })?;

    let mut response = Response::new(axum::body::Body::from(body_bytes));
    *response.status_mut() = status;
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    Ok(response)
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let url = room_url(&state, &slug, "post")?;
    proxy::proxy_to_service(&state.http, reqwest::Method::GET, &url, &headers, Bytes::new()).await
}

pub async fn get_comments(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let url = room_url(&state, &slug, "comments")?;
    proxy::proxy_to_service(&state.http, reqwest::Method::GET, &url, &headers, Bytes::new()).await
}

pub async fn post_comment(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    user: AuthUser,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    proxy_authed_post(&state, &user, &slug, "comments", &headers, body).await
}

pub async fn change_password(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    user: AuthUser,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    proxy_authed_post(&state, &user, &slug, "change-password", &headers, body).await
}

pub async fn reset(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    user: AuthUser,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    proxy_authed_post(&state, &user, &slug, "reset", &headers, Bytes::new()).await
}
