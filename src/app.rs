use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Info, OpenApi},
};
use axum::{
    extract::{Path, Query}, http::StatusCode, response::{IntoResponse, Json}, Extension, Router
};
use axum_extra::extract::WithRejection;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::err::ApiError;

#[derive(Deserialize, JsonSchema, Debug)]
struct HelloReq {
    name: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
struct Pagination {
    page: u32,
    count: u32,
}

#[derive(Serialize, JsonSchema, Debug)]
struct HelloRes {
    msg: String,
}

#[derive(Serialize, JsonSchema, Debug)]
struct UserRes {
    msg: String,
    user_id: String,
}

#[derive(Serialize, JsonSchema, Debug)]
struct PageRes {
    from: u32,
    to: u32,
}

pub(crate) fn init_app() -> Router {
    let api = ApiRouter::new()
        .api_route("/", get(root))
        .api_route("/health", get(health))
        .api_route("/hello", post(hello))
        .api_route("/user/:id", get(user_id))
        .api_route("/list", get(page))
        .api_route("/error", get(server_err))
        .route("/doc/api.json", get(serve_docs))
        // .method_not_allowed_fallback(bad_method)
        .fallback(not_found);

    let mut doc = OpenApi {
        info: Info {
            description: Some("An example Axum API for AWS Lambda".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };
    
    api.finish_api(&mut doc).layer(Extension(doc)).method_not_allowed_fallback(bad_method)
}

async fn root() -> Json<Value> {
    Json(json!({ "hello": "POST /hello", "health": "GET /health" }))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "OK" }))
}

async fn hello(
    WithRejection(body, _): WithRejection<Json<HelloReq>, ApiError>,
) -> impl IntoApiResponse {
    println!("Got request {:?}", body);

    let res = HelloRes {
        msg: format!("Hello there, {}", body.name),
    };

    println!("Response created! {:?}", res);

    (StatusCode::CREATED, Json(res))
}

async fn user_id(
    WithRejection(user_id, _): WithRejection<Path<String>, ApiError>,
) -> impl IntoApiResponse {
    let res = UserRes {
        msg: "User ID accepted".into(),
        user_id: user_id.0,
    };

    (StatusCode::OK, Json(res))
}

async fn page(
    WithRejection(pagination, _): WithRejection<Query<Pagination>, ApiError>,
) -> impl IntoApiResponse {
    let res = PageRes {
        from: pagination.page * pagination.count,
        to: (pagination.page + 1) * pagination.count - 1,
    };

    (StatusCode::OK, Json(res))
}

async fn server_err() -> impl IntoApiResponse {
    ApiError::Internal.into_response()
}

async fn bad_method() -> impl IntoApiResponse {
    ApiError::BadMethod.into_response()
}

async fn not_found() -> impl IntoApiResponse {
    ApiError::NotFound.into_response()
}

async fn serve_docs(
    Extension(api): Extension<OpenApi>,
) -> impl IntoApiResponse {
    Json(api)
}
