use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    BadGateway {
        error: &'static str,
        message: String,
    },
    Internal {
        error: &'static str,
        message: String,
    },
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

impl ApiError {
    pub fn bad_gateway(error: &'static str, message: impl Into<String>) -> Self {
        Self::BadGateway {
            error,
            message: message.into(),
        }
    }

    pub fn internal(error: &'static str, message: impl Into<String>) -> Self {
        Self::Internal {
            error,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            ApiError::BadGateway { error, message } => (StatusCode::BAD_GATEWAY, error, message),
            ApiError::Internal { error, message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, error, message)
            }
        };

        let body = Json(ErrorResponse { error, message });

        (status, body).into_response()
    }
}
