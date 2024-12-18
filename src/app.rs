use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::err::ApiError;

#[derive(Deserialize, Serialize, Debug)]
struct HelloReq {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct HelloRes {
    msg: String,
}

pub(crate) fn init_app() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/hello", post(hello))
        .route("/error", get(server_err))
        .method_not_allowed_fallback(bad_method)
        .fallback(not_found);

    app
}

async fn root() -> Json<Value> {
    Json(json!({ "hello": "POST /hello", "health": "GET /health" }))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "OK" }))
}

async fn hello(
    WithRejection(body, _): WithRejection<Json<HelloReq>, ApiError>,
) -> impl IntoResponse {
    println!("Got request {:?}", body);

    let res = HelloRes {
        msg: format!("Hello there, {}", body.name),
    };

    println!("Response created! {:?}", res);

    (StatusCode::CREATED, Json(res))
}

async fn server_err() -> impl IntoResponse {
    ApiError::Internal
}

async fn bad_method() -> impl IntoResponse {
    ApiError::BadMethod
}

async fn not_found() -> impl IntoResponse {
    ApiError::NotFound
}
