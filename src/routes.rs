use crate::handlers::{
    batch_get_balances_handler, batch_get_dao_handler, batch_get_dao_proposal_ids_handler,
    batch_get_governance_token_ids_handler, batch_get_proposals_handler, batch_get_stakes_handler,
    create_profile_handler, create_token_info_handler, get_all_dao_ids_handler,
    get_all_proposal_ids_handler, get_creating_dao_proposal_ids_handler, get_funds_total_handler,
    get_pledgers_total_handler, get_profile_handler, get_stake_funds_total_handler,
    records_handler, update_profile_handler, upsert_profile_handler
};
use axum::routing::post;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/records", get(records_handler))
        .route("/profiles", get(get_profile_handler))
        .route("/dao_ids", get(get_all_dao_ids_handler))
        .route("/daos", get(batch_get_dao_handler))
        .route(
            "/governance_token_ids",
            get(batch_get_governance_token_ids_handler),
        )
        .route("/dao_proposal_ids", get(batch_get_dao_proposal_ids_handler))
        .route("/balances", get(batch_get_balances_handler))
        .route("/stakes", get(batch_get_stakes_handler))
        .route("/pledgers_total", get(get_pledgers_total_handler))
        .route("/stake_funds_total", get(get_stake_funds_total_handler))
        .route("/funds_total", get(get_funds_total_handler))
        .route(
            "/creating_dao_proposal_ids",
            get(get_creating_dao_proposal_ids_handler),
        )
        .route("/proposals_handler", get(batch_get_proposals_handler))
        .route("/all_proposal_ids", get(get_all_proposal_ids_handler))
        .route("/crate_profile", get(create_profile_handler))
        .route("/update_profile", get(update_profile_handler))
        .route("/upsert_profile", get(upsert_profile_handler))
        .route("/create_token_info", get(create_token_info_handler))
}
