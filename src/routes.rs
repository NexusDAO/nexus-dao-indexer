use crate::handlers::records_handler;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/records", get(records_handler))
}
