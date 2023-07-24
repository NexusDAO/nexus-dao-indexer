use crate::database::{get_balances_by_owner, get_pledgers_by_token_info_id, get_token_info_by_id};
use crate::models::{Balances, StakeAmounts};
use crate::program_handler::bhp256_hash_address;
use crate::{
    database::{
        get_all_dao_ids, get_all_proposal_ids, get_creating_dao_proposal_ids, get_dao_by_id,
        get_dao_proposal_ids_by_dao_id, get_funds_total, get_pledgers_total,
        get_profile_by_address, get_proposals_by_proposal_id, get_records_by_height,
        get_stake_funds_total, get_stakes_by_owner, insert_profile, insert_token_info,
        update_profile, upsert_profile, POOL,
    },
    models::{Daos, Input, Output, Profiles, Proposals, RespRecords, TokenInfos},
};
use axum::{
    extract::{Json as PostJson, Path, Query},
    response::Json,
};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use std::{collections::HashMap, str::FromStr};

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

pub async fn get_profile_handler(Path(address): Path<String>) -> Json<Profiles> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let profiles = get_profile_by_address(&mut conn, address).unwrap();

    Json(profiles)
}

pub async fn get_all_dao_ids_handler() -> Json<Vec<i64>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let dao_ids = get_all_dao_ids(&mut conn);
    Json(dao_ids.unwrap())
}

pub async fn batch_get_dao_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Daos>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_vec_dao: Vec<Daos> = Vec::new();
    let id_array: Vec<i64> =
        serde_json::from_str(&params.get("id-array").unwrap().to_string()).unwrap();

    for id in id_array {
        let dao = get_dao_by_id(&mut conn, id);
        match dao {
            Ok(dao) => {
                ret_vec_dao.push(dao);
            }
            Err(err) => {
                let empty_dao = Daos {
                    id: 0,
                    name: "".to_string(),
                    dao_type: 0,
                    creator: "".to_string(),
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

pub async fn batch_get_token_id_of_dao_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<i64>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_token_ids: Vec<i64> = Vec::new();
    let id_array: Vec<i64> =
        serde_json::from_str(&params.get("dao-id-array").unwrap().to_string()).unwrap();

    for id in id_array {
        let dao = get_dao_by_id(&mut conn, id);
        match dao {
            Ok(dao) => {
                ret_token_ids.push(dao.token_info_id);
            }
            Err(err) => {
                ret_token_ids.push(-1);
            }
        }
    }
    Json(ret_token_ids)
}

pub async fn batch_get_proposal_id_of_dao_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Vec<i64>>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_proposal_ids: Vec<Vec<i64>> = Vec::new();

    let id_array: Vec<i64> =
        serde_json::from_str(&params.get("dao-id-array").unwrap().to_string()).unwrap();

    for id in id_array {
        let proposal_ids = get_dao_proposal_ids_by_dao_id(&mut conn, id);
        match proposal_ids {
            Ok(ids) => ret_proposal_ids.push(ids),
            Err(err) => {
                let nil: Vec<i64> = Vec::new();
                ret_proposal_ids.push(nil);
            }
        }
    }
    Json(ret_proposal_ids)
}

pub async fn batch_get_token_info_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<TokenInfos>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_token_infos: Vec<TokenInfos> = Vec::new();
    let id_array: Vec<i64> =
        serde_json::from_str(&params.get("id-array").unwrap().to_string()).unwrap();

    for id in id_array {
        let token_info = get_token_info_by_id(&mut conn, id);
        match token_info {
            Ok(token_info) => {
                ret_token_infos.push(token_info);
            }
            Err(err) => {
                ret_token_infos.push(TokenInfos {
                    id: 0,
                    name: "".to_string(),
                    symbol: "".to_string(),
                    supply: 0,
                    decimals: 0,
                    max_mint_amount: 0,
                    minted_amount: 0,
                    dao_id: 0,
                    only_creator_can_mint: false,
                });
            }
        }
    }

    Json(ret_token_infos)
}

pub async fn get_balances_handler(Path(address): Path<String>) -> Json<Vec<Balances>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let hash_addr = bhp256_hash_address(&address).unwrap();
    let ret_balances = get_balances_by_owner(&mut conn, hash_addr.to_string()).unwrap();

    Json(ret_balances)
}

pub async fn get_stakes_handler(Path(address): Path<String>) -> Json<Vec<StakeAmounts>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let hash_addr = bhp256_hash_address(&address).unwrap();
    let ret_stakes = get_stakes_by_owner(&mut conn, hash_addr.to_string()).unwrap();

    Json(ret_stakes)
}

pub async fn batch_get_pledgers_by_token_info_id(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<i64>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_pledgers: Vec<i64> = Vec::new();
    let token_info_id_array: Vec<i64> =
        serde_json::from_str(&params.get("token-info-id-array").unwrap().to_string()).unwrap();
    for id in token_info_id_array {
        let pledgers = get_pledgers_by_token_info_id(&mut conn, id).unwrap_or_default();
        ret_pledgers.push(pledgers);
    }
    Json(ret_pledgers)
}

pub async fn get_pledgers_total_handler() -> String {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let ret_pledgers_total = get_pledgers_total(&mut conn).unwrap();
    ret_pledgers_total
}

pub async fn get_stake_funds_total_handler() -> String {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let ret_stake_funds_total = get_stake_funds_total(&mut conn).unwrap();
    ret_stake_funds_total
}

pub async fn get_funds_total_handler() -> String {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let ret_stake_funds_total = get_funds_total(&mut conn).unwrap();
    ret_stake_funds_total
}

pub async fn get_creating_dao_proposal_ids_handler() -> Json<Vec<i64>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let prop_id = get_creating_dao_proposal_ids(&mut conn).unwrap();
    Json(prop_id)
}

pub async fn batch_get_proposals_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Vec<Proposals>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_proposals: Vec<Proposals> = Vec::new();
    let proposal_id_array: Vec<i64> =
        serde_json::from_str(&params.get("id-array").unwrap().to_string()).unwrap();
    for id in proposal_id_array {
        let proposal = get_proposals_by_proposal_id(&mut conn, id);
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
                    type_: 0,
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

pub async fn get_all_proposal_ids_handler() -> Json<Vec<i64>> {
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
