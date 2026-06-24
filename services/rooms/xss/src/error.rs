use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    Internal {
        error: &'static str,
        message: String,
    },
    Unauthorized {
        error: &'static str,
        message: String,
    },
    BadRequest {
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
    pub fn internal(error: &'static str, message: impl Into<String>) -> Self {
        Self::Internal {
            error,
            message: message.into(),
        }
    }

    pub fn unauthorized(error: &'static str, message: impl Into<String>) -> Self {
        Self::Unauthorized {
            error,
            message: message.into(),
        }
    }

    pub fn bad_request(error: &'static str, message: impl Into<String>) -> Self {
        Self::BadRequest {
            error,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            ApiError::Internal { error, message } => {
                (StatusCode::INTERNAL_SERVER_ERROR, error, message)
            }
            ApiError::Unauthorized { error, message } => {
                (StatusCode::UNAUTHORIZED, error, message)
            }
            ApiError::BadRequest { error, message } => (StatusCode::BAD_REQUEST, error, message),
        };

        (status, Json(ErrorResponse { error, message })).into_response()
    }
}
