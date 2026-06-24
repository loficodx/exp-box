use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};

use super::proxy;
use crate::{api::error::ApiError, auth::AuthUser, state::AppState};

fn xss_url(state: &AppState, path: &str) -> Result<String, ApiError> {
    let room = state.rooms.get("xss").ok_or_else(|| {
        ApiError::bad_gateway("xss_room_not_configured", "xss room target is not configured")
    })?;
    Ok(room.action_url(path))
}

pub async fn get_post(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let url = xss_url(&state, "post")?;
    proxy::proxy_to_service(&state.http, reqwest::Method::GET, &url, &headers, Bytes::new()).await
}

pub async fn get_comments(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let url = xss_url(&state, "comments")?;
    proxy::proxy_to_service(&state.http, reqwest::Method::GET, &url, &headers, Bytes::new()).await
}

pub async fn post_comment(
    State(state): State<AppState>,
    user: AuthUser,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let url = xss_url(&state, "comments")?;

    let mut builder = state.http.post(&url);

    if let Some(ct) = headers.get(axum::http::header::CONTENT_TYPE) {
        builder = builder.header("Content-Type", ct.as_bytes());
    }

    builder = builder
        .header("X-Lab-User-Id", &user.user_id)
        .header("X-Lab-Username", &user.username)
        .body(body);

    let resp = builder.send().await.map_err(|e| {
        ApiError::bad_gateway("xss_room_unavailable", format!("room-xss unreachable: {e}"))
    })?;

    let status =
        StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let body_bytes = resp.bytes().await.map_err(|e| {
        ApiError::bad_gateway(
            "xss_room_response_failed",
            format!("failed to read room-xss response: {e}"),
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
