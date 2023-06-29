use crate::handlers::{records_handler, get_profile_handler, get_all_dao_ids_handler, batch_get_dao_handler, create_profile_handler, create_token_info_handler};
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/records", get(records_handler))
        .route("/profiles", get(get_profile_handler))
        .route("/dao_ids", get(get_all_dao_ids_handler))
        .route("/daos", get(batch_get_dao_handler))
        .route("/crate_profile", get(create_profile_handler))
        .route("/create_token_info", get(create_token_info_handler))
}

