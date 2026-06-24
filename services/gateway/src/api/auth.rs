use axum::{
    body::Bytes,
    extract::State,
    http::HeaderMap,
    response::Response,
};

use super::proxy;
use crate::{api::error::ApiError, state::AppState};

pub async fn register(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let url = format!("{}/register", state.auth_service_url);
    proxy::proxy_to_service(&state.http, reqwest::Method::POST, &url, &headers, body).await
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let url = format!("{}/login", state.auth_service_url);
    proxy::proxy_to_service(&state.http, reqwest::Method::POST, &url, &headers, body).await
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let url = format!("{}/logout", state.auth_service_url);
    proxy::proxy_to_service(&state.http, reqwest::Method::POST, &url, &headers, body).await
}

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let url = format!("{}/me", state.auth_service_url);
    proxy::proxy_to_service(
        &state.http,
        reqwest::Method::GET,
        &url,
        &headers,
        Bytes::new(),
    )
    .await
}
