use crate::handlers::{get_ratifications, ok};
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(ok).post(ok))
        .route("/ratifications/:height", get(get_ratifications))
}
