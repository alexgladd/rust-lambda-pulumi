use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub(crate) enum ApiError {
    #[error(transparent)]
    JsonDeserialization(#[from] JsonRejection),

    #[error(transparent)]
    PathDeserialization(#[from] PathRejection),

    #[error(transparent)]
    QueryDeserialization(#[from] QueryRejection),

    #[error("Resource not found")]
    NotFound,

    #[error("Method not allowed")]
    BadMethod,

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let timestamp = get_now_str();
        let (code, body) = match self {
            ApiError::JsonDeserialization(r) => (r.status(), r.into()),
            ApiError::PathDeserialization(r) => (r.status(), r.into()),
            ApiError::QueryDeserialization(r) => (r.status(), r.into()),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ApiErrorPayload {
                    message: self.to_string(),
                    timestamp,
                },
            ),
            ApiError::BadMethod => (
                StatusCode::METHOD_NOT_ALLOWED,
                ApiErrorPayload {
                    message: self.to_string(),
                    timestamp,
                },
            ),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorPayload {
                    message: self.to_string(),
                    timestamp,
                },
            ),
        };

        (code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct ApiErrorPayload {
    message: String,
    timestamp: String,
}

impl From<JsonRejection> for ApiErrorPayload {
    fn from(value: JsonRejection) -> Self {
        let message = match value {
            JsonRejection::JsonDataError(_) => "Model deserialization failed",
            JsonRejection::JsonSyntaxError(_) => "Invalid JSON syntax",
            JsonRejection::MissingJsonContentType(_) => "Invalid Content-Type header",
            JsonRejection::BytesRejection(_) => "Unable to process request body",
            _ => "Unknown JSON body error",
        }
        .into();

        Self {
            message,
            timestamp: get_now_str(),
        }
    }
}

impl From<PathRejection> for ApiErrorPayload {
    fn from(value: PathRejection) -> Self {
        let message = match value {
            PathRejection::FailedToDeserializePathParams(_) => {
                "Path parameter deserialization failed"
            }
            PathRejection::MissingPathParams(_) => "Missing path parameter(s)",
            _ => "Unknown URL path error",
        }
        .into();

        Self {
            message,
            timestamp: get_now_str(),
        }
    }
}

impl From<QueryRejection> for ApiErrorPayload {
    fn from(value: QueryRejection) -> Self {
        let message = match value {
            QueryRejection::FailedToDeserializeQueryString(_) => "Query deserialization failed",
            _ => "Unknown URL query error",
        }
        .into();

        Self {
            message,
            timestamp: get_now_str(),
        }
    }
}

fn get_now_str() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
