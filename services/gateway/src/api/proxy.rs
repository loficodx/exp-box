use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};

use super::error::ApiError;

/// Forward a request to an internal service, preserving Cookie and Set-Cookie headers.
/// Status is passed through verbatim — only network/read failures become 502.
pub async fn proxy_to_service(
    http: &reqwest::Client,
    method: reqwest::Method,
    url: &str,
    headers: &HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let mut builder = http.request(method, url);

    if let Some(cookie) = headers.get(axum::http::header::COOKIE) {
        builder = builder.header("Cookie", cookie.as_bytes());
    }

    if !body.is_empty() {
        if let Some(ct) = headers.get(axum::http::header::CONTENT_TYPE) {
            builder = builder.header("Content-Type", ct.as_bytes());
        }
        builder = builder.body(body);
    }

    let resp = builder.send().await.map_err(|e| {
        ApiError::bad_gateway("service_unavailable", format!("upstream unreachable: {e}"))
    })?;

    let status = StatusCode::from_u16(resp.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let set_cookies: Vec<HeaderValue> = resp
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|v| HeaderValue::from_bytes(v.as_bytes()).ok())
        .collect();

    let body_bytes = resp.bytes().await.map_err(|e| {
        ApiError::bad_gateway(
            "service_response_failed",
            format!("failed to read upstream response: {e}"),
        )
    })?;

    let mut response = Response::new(axum::body::Body::from(body_bytes));
    *response.status_mut() = status;
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    for v in set_cookies {
        response
            .headers_mut()
            .append(axum::http::header::SET_COOKIE, v);
    }

    Ok(response)
}
