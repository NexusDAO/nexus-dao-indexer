use crate::handlers::{records_handler, get_profile_handler};
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/records", get(records_handler));
    Router::new().route("/profiles", get(get_profile_handler))
}

