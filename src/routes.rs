use crate::handlers::{
    batch_get_dao_handler, batch_get_pledgers_by_token_info_id,
    batch_get_proposal_id_of_dao_handler, batch_get_proposals_handler,
    batch_get_token_id_of_dao_handler, batch_get_token_info_handler, create_profile_handler,
    create_token_info_handler, get_all_dao_ids_handler, get_all_proposal_ids_handler,
    get_balances_handler, get_creating_dao_proposal_ids_handler, get_funds_total_handler,
    get_pledgers_total_handler, get_profile_handler, get_stake_funds_total_handler,
    get_stakes_handler, records_handler, update_profile_handler, upsert_profile_handler,
};
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/records", get(records_handler))
        .route("/profile/:address", get(get_profile_handler))
        .route("/all-dao-ids", get(get_all_dao_ids_handler))
        .route("/daos", get(batch_get_dao_handler))
        .route("/token-ids", get(batch_get_token_id_of_dao_handler))
        .route("/token-info", get(batch_get_token_info_handler))
        .route(
            "/dao_proposal_ids",
            get(batch_get_proposal_id_of_dao_handler),
        )
        .route("/balances/:address", get(get_balances_handler))
        .route("/stakes/:address", get(get_stakes_handler))
        .route("/pledgers", get(batch_get_pledgers_by_token_info_id))
        .route("/pledgers-total", get(get_pledgers_total_handler))
        .route("/stake-funds-total", get(get_stake_funds_total_handler))
        .route("/funds-total", get(get_funds_total_handler))
        .route(
            "/proposal-ids/creating-dao",
            get(get_creating_dao_proposal_ids_handler),
        )
        .route("/proposals", get(batch_get_proposals_handler))
        .route("/all-proposal-ids", get(get_all_proposal_ids_handler))
        .route("/crate_profile", get(create_profile_handler))
        .route("/update_profile", get(update_profile_handler))
        .route("/upsert_profile", get(upsert_profile_handler))
        .route("/create_token_info", get(create_token_info_handler))
}
