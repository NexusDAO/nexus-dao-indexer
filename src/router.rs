use crate::graphql::create_schema;
use crate::models::Ratify;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::GraphQL;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Json;
use axum::{routing::get, Router};
use http::{Response, StatusCode};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/", get(health_handler).post(health_handler))
        .route("/ratifications/:height", get(get_ratifications))
        .route(
            "/graphql",
            get(graphiql).post_service(GraphQL::new(create_schema())),
        )
}

async fn health_handler() -> Response<String> {
    Response::new(String::from("ok"))
}

async fn get_ratifications(Path(height): Path<u32>) -> (StatusCode, Json<Value>) {
    match Ratify::list_by_height(i64::from(height)) {
        Ok(result) => (StatusCode::OK, Json(json!(result))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null)),
    }
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
