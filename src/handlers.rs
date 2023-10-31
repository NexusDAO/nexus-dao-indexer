use crate::database;
use axum::extract::Path;
use axum::Json;
use http::{Response, StatusCode};
use serde_json::{json, Value};

pub async fn ok() -> Response<String> {
    Response::new(String::from("ok"))
}

pub async fn get_ratifications(Path(height): Path<u32>) -> (StatusCode, Json<Value>) {
    match database::get_ratifications(i64::from(height)) {
        Ok(result) => (StatusCode::OK, Json(json!(result))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null)),
    }
}
