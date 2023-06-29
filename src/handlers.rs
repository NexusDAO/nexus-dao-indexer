use crate::{
    database::{
        create_profile, get_all_dao_ids, get_dao_by_id, get_dao_proposal_ids_by_dao_id,
        get_profile_by_address, get_records_by_height, update_profile, POOL, insert_token_info
    },
    models::{Daos, Output, Profiles, RespProfile, RespRecords, TokenInfos},
    schema::profiles::avatar,
};
use axum::{extract::Query, response::Json};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use std::{collections::HashMap, default, str::FromStr};
use diesel::row::NamedRow;

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
            let outputs: Vec<Output> = serde_json::from_str(&record.outputs).unwrap();
            let record_values = outputs
                .into_iter()
                .filter_map(|output| {
                    if output.r#type.eq("record") {
                        Some(output.value)
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

pub async fn create_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let addr = params.get("address").unwrap().to_string();
    let names = params.get("name").unwrap().to_string();
    let avatars = params.get("avatar").unwrap().to_string();
    let bios = params.get("bio").unwrap().to_string();
    let profile = Profiles{address: addr, name: names, avatar: avatars, bio: bios};

    let status = create_profile(&mut conn, profile).unwrap();
    Json(status.to_string())
}

pub async fn create_token_info_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
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

    let token_info = TokenInfos{
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
    let update_addr = params.get("updata_address").unwrap();

    let profile = Profiles{address: addr, name: names, avatar: avatars, bio: bios};

    let status = update_profile(&mut conn, profile, update_addr).unwrap();
    Json(status.to_string())
}

pub async fn get_all_dao_ids_handler() -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let dao_ids = get_all_dao_ids(&mut conn);
    Json(dao_ids.unwrap())
}

pub async fn batch_get_dao_handler(
    Query(params): Query<HashMap<String, Vec<String>>>,
) -> Json<Vec<Daos>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_vec_dao: Vec<Daos> = Vec::new();
    let id_array = params.get("dao_id_array").unwrap();

    for id in id_array {
        let parsed_id = string_to_i64(id);
        let dao = get_dao_by_id(&mut conn, parsed_id);
        match dao {
            Ok(dao) => {
                ret_vec_dao.push(dao);
            },
            Err(err) =>{
                println!("error of {}", err)
            }
        }
    }
    Json(ret_vec_dao)
}

pub async fn batch_get_governance_token_ids_handler(
    Query(params): Query<HashMap<String, Vec<String>>>,
) -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_token_ids: Vec<String> = Vec::new();

    let id_array = params.get("dao_id_array").unwrap();

    for id in id_array {
        let parsed_id = string_to_i64(id);
        let dao = get_dao_by_id(&mut conn, parsed_id);
        match dao {
            Ok(dao) => {
                ret_token_ids.push(dao.token_info_id.to_string());
            },
            Err(err) =>{
                println!("error of {}", err)
            }
        }
    }
    Json(ret_token_ids)
}

pub async fn batch_get_dao_proposal_ids_handler(
    Query(params): Query<HashMap<String, Vec<String>>>,
) -> Json<Vec<Vec<String>>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_proposal_ids: Vec<Vec<String>> = Vec::new();

    let id_array = params.get("dao_id_array").unwrap();

    for id in id_array {
        let parsed_id = string_to_i64(id);
        let proposal_ids = get_dao_proposal_ids_by_dao_id(&mut conn, parsed_id).unwrap();
        ret_proposal_ids.push(proposal_ids)
    }
    Json(ret_proposal_ids)
}

// pub async fn create_dao_handler(
//     Query(params): Query<HashMap<String, String>>,
// ) -> Json<String> {
//     let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
//     let dao_id = params.get("dao_id").unwrap();
//     let dao_name = params.get("dao_name").unwrap();
//     let dao_type = params.get("dao_type").unwrap();
//     let dao_creater = params.get("dao_creater").unwrap();
//     let dao_token_info_id = params.get("dao_token_info_id").unwrap();
//     let dao_icon = params.get("dao_icon").unwrap();
//     let dao_description = params.get("dao_description").unwrap();
//     let dao_official_link = params.get("dao_official_link").unwrap();
//     let dao_proposal_count = params.get("dao_proposal_count").unwrap();
//     let dao_pass_proposal_count = params.get("dao_pass_proposal_count").unwrap();
//     let dao_vote_count = params.get("dao_vote_count").unwrap();
//     let dao_passed_votes_proportion = params.get("dao_passed_votes_proportion").unwrap();
//     let dao_passed_tokens_proportion = params.get("dao_passed_tokens_proportion").unwrap();
//
//
//     Json("OK".to_string())
// }


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