use crate::models::{Balances, StakeAmounts};
use crate::schema::daos::dsl::daos;
use crate::{
    database::{
        create_profile, get_all_dao_ids, get_all_proposal_ids, get_balances,
        get_creating_dao_proposal_ids, get_dao_by_id, get_dao_proposal_ids_by_dao_id,
        get_funds_total, get_pledgers_total, get_profile_by_address, get_proposals_by_proposal_id,
        get_records_by_height, get_stake_funds_total, get_stakes, insert_token_info,
        update_profile, POOL,
    },
    models::{Daos, Input, Output, Profiles, Proposals, RespProfile, RespRecords, TokenInfos},
    schema::profiles::avatar,
};
use axum::{extract::Query, response::Json};
use diesel::row::NamedRow;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use std::{collections::HashMap, default, str::FromStr};
use snarkvm::circuit::IntegerProperties;

pub async fn records_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<RespRecords>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let default_start_block = "0".to_string();
    let default_end_block = i64::MAX.to_string();
    let start_block = params.get("start_block").unwrap_or(&default_start_block);
    let start_block = i64::from_str(start_block).unwrap_or(0);
    let end_block = params.get("end_block").unwrap_or(&default_end_block);
    let end_block = i64::from_str(&end_block).unwrap_or(i64::MAX);

    let records = get_records_by_height(&mut conn, start_block, end_block).unwrap();

    let results = records
        .iter()
        .map(|record| {
            let inputs: Vec<Input> = serde_json::from_str(&record.inputs).unwrap();
            let outputs: Vec<Output> = serde_json::from_str(&record.outputs).unwrap();
            let record_values = outputs
                .iter()
                .filter_map(|output| {
                    if output.r#type.eq("record") {
                        Some(output.value.clone())
                    } else {
                        None
                    }
                })
                .collect();

            RespRecords {
                records: record_values,
                transaction_id: record.transaction_id.clone(),
                transition_id: record.transition_id.clone(),
                network: record.network,
                height: record.height,
                timestamp: record.timestamp,
                inputs: inputs,
                outputs: outputs,
            }
        })
        .collect::<Vec<RespRecords>>();

    Json(results)
}

pub async fn get_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<Profiles> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let address = params.get("address").unwrap();

    let profiles = get_profile_by_address(&mut conn, address.to_string()).unwrap();

    Json(profiles)
}

pub async fn get_all_dao_ids_handler() -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let dao_ids = get_all_dao_ids(&mut conn);
    Json(dao_ids.unwrap())
}

pub async fn batch_get_dao_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Daos>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_vec_dao: Vec<Daos> = Vec::new();
    let id_array: Vec<String> =
        serde_json::from_str(&params.get("dao_id_array").unwrap().to_string()).unwrap();

    for id in id_array {
        let parsed_id = string_to_i64(&id);
        let dao = get_dao_by_id(&mut conn, parsed_id);
        match dao {
            Ok(dao) => {
                ret_vec_dao.push(dao);
            }
            Err(err) => {
                let empty_dao = Daos {
                    id: 0,
                    name: "".to_string(),
                    dao_type: 0,
                    creater: "".to_string(),
                    token_info_id: 0,
                    icon: "".to_string(),
                    description: "".to_string(),
                    official_link: "".to_string(),
                    proposal_count: 0,
                    pass_proposal_count: 0,
                    vote_count: 0,
                    passed_votes_proportion: 0,
                    passed_tokens_proportion: 0,
                };

                ret_vec_dao.push(empty_dao);
            }
        }
    }
    Json(ret_vec_dao)
}

pub async fn batch_get_governance_token_ids_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_token_ids: Vec<String> = Vec::new();
    let id_array: Vec<String> =
        serde_json::from_str(&params.get("dao_id_array").unwrap().to_string()).unwrap();

    for id in id_array {
        let parsed_id = string_to_i64(&id);
        let dao = get_dao_by_id(&mut conn, parsed_id);
        match dao {
            Ok(dao) => {
                ret_token_ids.push(dao.token_info_id.to_string());
            }
            Err(err) => {
                ret_token_ids.push("nil".to_string());
            }
        }
    }
    Json(ret_token_ids)
}

pub async fn batch_get_dao_proposal_ids_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Vec<String>>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_proposal_ids: Vec<Vec<String>> = Vec::new();

    let id_array: Vec<String> =
        serde_json::from_str(&params.get("dao_id_array").unwrap().to_string()).unwrap();

    for id in id_array {
        let parsed_id = string_to_i64(&id);
        let proposal_ids = get_dao_proposal_ids_by_dao_id(&mut conn, parsed_id);
        match proposal_ids {
            Ok(ids) => ret_proposal_ids.push(ids),
            Err(err) => {
                let nil: Vec<String> = Vec::new();
                ret_proposal_ids.push(nil)
            }
        }
    }
    Json(ret_proposal_ids)
}

pub async fn batch_get_balances_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Balances>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let key = params.get("key").unwrap();
    let ret_blances = get_balances_by_key(&mut conn, key.to_string()).unwrap();

    Json(ret_blances)
}

pub async fn batch_get_stakes_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<StakeAmounts>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let key = params.get("key").unwrap();
    let ret_stakes = get_stakes_by_key(&mut conn, key.to_string()).unwrap();

    Json(ret_stakes)
}

pub async fn get_pledgers_total_handler() -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let ret_pledgers_total = get_pledgers_total(&mut conn).unwrap();
    Json(ret_pledgers_total)
}

pub async fn get_stake_funds_total_handler() -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let ret_stake_funds_total = get_stake_funds_total(&mut conn).unwrap();
    Json(ret_stake_funds_total)
}

pub async fn get_funds_total_handler() -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let ret_stake_funds_total = get_funds_total(&mut conn).unwrap();
    Json(ret_stake_funds_total)
}

pub async fn get_creating_dao_proposal_ids_handler() -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let prop_id = get_creating_dao_proposal_ids(&mut conn).unwrap();
    Json(prop_id)
}

pub async fn batch_get_proposals_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Proposals>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_proposals: Vec<Proposals> = Vec::new();
    let proposal_id_array: Vec<String> =
        serde_json::from_str(&params.get("proposal_id_array").unwrap().to_string()).unwrap();
    for id in proposal_id_array {
        let parse_id = string_to_i64(&id);
        let proposal = get_proposals_by_proposal_id(&mut conn, parse_id);
        match proposal {
            Ok(proposal) => ret_proposals.push(proposal),
            Err(err) => {
                let empty_proposals = Proposals {
                    id: 0,
                    title: "".to_string(),
                    proposer: "".to_string(),
                    summary: "".to_string(),
                    body: "".to_string(),
                    dao_id: 0,
                    created: 0,
                    duration: 0,
                    proposer_type: 0,
                    adopt: 0,
                    reject: 0,
                    status: 0,
                };
                ret_proposals.push(empty_proposals)
            }
        }
    }
    Json(ret_proposals)
}

pub async fn get_all_proposal_ids_handler() -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let ret_proposal_ids = get_all_proposal_ids(&mut conn).unwrap();
    Json(ret_proposal_ids)
}

pub fn string_to_i64(input: &String) -> i64 {
    let parsed_id: Result<i64, _> = input.parse();
    match parsed_id {
        Ok(number) => {
            return number;
        }
        Err(error) => {
            panic!("Parsing error: {:?}", error);
        }
    }
}

pub async fn create_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let addr = params.get("address").unwrap().to_string();
    let names = params.get("name").unwrap().to_string();
    let avatars = params.get("avatar").unwrap().to_string();
    let bios = params.get("bio").unwrap().to_string();
    let profile = Profiles {
        address: addr,
        name: names,
        avatar: avatars,
        bio: bios,
    };

    let status = insert_profile(&mut conn, profile).unwrap();
    Json(status.to_string())
}

pub async fn create_token_info_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let id = params.get("id").unwrap().to_string();
    let name = params.get("name").unwrap().to_string();
    let symbol = params.get("symbol").unwrap().to_string();
    let supply = params.get("supply").unwrap().to_string();
    let decimals = params.get("decimals").unwrap().to_string();
    let max_mint_amount = params.get("max_mint_amount").unwrap().to_string();
    let minted_amount = params.get("minted_amount").unwrap().to_string();
    let dao_id = params.get("dao_id").unwrap().to_string();
    let mut only_creator_can_mints = false;
    if params.get("only_creator_can_mint").is_some() {
        only_creator_can_mints = true;
    };

    let token_info = TokenInfos {
        id: string_to_i64(&id),
        name,
        symbol,
        supply: string_to_i64(&supply),
        decimals: string_to_i64(&decimals),
        max_mint_amount: string_to_i64(&max_mint_amount),
        minted_amount: string_to_i64(&minted_amount),
        dao_id: string_to_i64(&dao_id),
        only_creator_can_mint: only_creator_can_mints,
    };

    let status = insert_token_info(&mut conn, token_info).unwrap();
    Json(status.to_string())
}

pub async fn update_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let addr = params.get("address").unwrap().to_string();
    let names = params.get("name").unwrap().to_string();
    let avatars = params.get("avatar").unwrap().to_string();
    let bios = params.get("bio").unwrap().to_string();

    let profile = Profiles {
        address: addr,
        name: names,
        avatar: avatars,
        bio: bios,
    };

    let status = update_profile(&mut conn, profile).unwrap();
    Json(status.to_string())
}

pub async fn upsert_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let addr = params.get("address").unwrap().to_string();
    let names = params.get("name").unwrap().to_string();
    let avatars = params.get("avatar").unwrap().to_string();
    let bios = params.get("bio").unwrap().to_string();

    let profile = Profiles {
        address: addr,
        name: names,
        avatar: avatars,
        bio: bios,
    };

    let status = upsert_profile(&mut conn, profile).unwrap();
    Json(status.to_string())
}
