use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub(crate) enum ApiError {
    #[error(transparent)]
    JsonDeserialization(#[from] JsonRejection),

    #[error("Resource not found")]
    NotFound,

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, body) = match self {
            ApiError::JsonDeserialization(r) => (r.status(), r.into()),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ApiErrorPayload {
                    message: self.to_string(),
                    timestamp: Utc::now(),
                },
            ),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorPayload {
                    message: self.to_string(),
                    timestamp: Utc::now(),
                },
            ),
            // _ => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     ApiErrorPayload {
            //         message: format!("Unknown error: {}", self),
            //         timestamp: Utc::now(),
            //     },
            // ),
        };

        (code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiErrorPayload {
    message: String,
    timestamp: DateTime<Utc>,
}

impl From<JsonRejection> for ApiErrorPayload {
    fn from(value: JsonRejection) -> Self {
        let message = match value {
            JsonRejection::JsonDataError(_) => "Model deserialization failed",
            JsonRejection::JsonSyntaxError(_) => "Invalid JSON syntax",
            JsonRejection::MissingJsonContentType(_) => "Invalid Content-Type header",
            JsonRejection::BytesRejection(_) => "Unable to process request body",
            _ => "Unknown error",
        }
        .into();

        Self {
            message,
            timestamp: Utc::now(),
        }
    }
}
