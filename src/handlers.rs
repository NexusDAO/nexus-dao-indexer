use crate::{
    database::{
        create_profile, get_all_dao_ids, get_dao_by_id, get_dao_proposal_ids_by_dao_id,
        get_profile_by_address, get_records_by_height, update_profile, POOL,
    },
    models::{Dao, Output, Profile, RespProfile, RespRecords, TokenInfo},
    schema::profile::avatar,
};
use axum::{extract::Query, response::Json};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use std::{collections::HashMap, default, str::FromStr};

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

pub async fn get_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<Profile> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let default_addr = "0".to_string();
    let address = params.get("address").unwrap_or(&default_addr);

    let profiles = get_profile_by_address(&mut conn, address).unwrap();

    Json(profiles)
}

pub async fn create_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let default_addr = "0".to_string();
    let default_name = "0".to_string();
    let default_avatar = "0".to_string();
    let default_bio = "0".to_string();

    let addr = params.get("address").unwrap_or(&default_addr);
    let names = params.get("name").unwrap_or(&default_name);
    let avatars = params.get("avatar").unwrap_or(&default_avatar);
    let bios = params.get("bio").unwrap_or(&default_bio);

    let status = create_profile(&mut conn, addr, names, avatars, bios).unwrap();
    Json(status.to_string())
}

pub async fn update_profile_handler(Query(params): Query<HashMap<String, String>>) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let default_addr = "0".to_string();
    let default_name = "0".to_string();
    let default_avatar = "0".to_string();
    let default_bio = "0".to_string();

    let addr = params.get("address").unwrap_or(&default_addr);
    let names = params.get("name").unwrap_or(&default_name);
    let avatars = params.get("avatar").unwrap_or(&default_avatar);
    let bios = params.get("bio").unwrap_or(&default_bio);

    let status = update_profile(&mut conn, addr, names, avatars, bios).unwrap();
    Json(status.to_string())
}

pub async fn get_all_dao_ids_handler() -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();

    let dao_ids = get_all_dao_ids(&mut conn);
    Json(dao_ids.unwrap())
}

pub async fn batch_get_dao_handler(
    Query(params): Query<HashMap<String, Vec<String>>>,
) -> Json<Vec<Dao>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_vec_dao: Vec<Dao> = Vec::new();
    let id_array = params.get("daoIdArray").unwrap();

    for id in id_array {
        let dao = get_dao_by_id(&mut conn, id).unwrap();
        ret_vec_dao.push(dao);
    }

    Json(ret_vec_dao)
}

pub async fn batch_get_governance_token_ids_handler(
    Query(params): Query<HashMap<String, Vec<String>>>,
) -> Json<Vec<String>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let mut ret_token_ids: Vec<String> = Vec::new();

    let id_array = params.get("daoIdArray").unwrap();

    for id in id_array {
        let parsed_id : Result<i64, _> = id.parse();
        match parsed_id {
            Ok(number) => {
                let dao = get_dao_by_id(&mut conn, number).unwrap();
                // let token_info: TokenInfo = serde_json::from_str(&dao.token_info_id).unwrap();
                ret_token_ids.push(dao.token_info_id.to_string());

            }
            Err(error) => {
                println!("Parsing error: {:?}", error);
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

    let id_array = params.get("daoIdArray").unwrap();

    for id in id_array {
        let parsed_id : Result<i64, _> = id.parse();
        match parsed_id {
            Ok(number) => {
                let proposal_ids = get_dao_proposal_ids_by_dao_id(&mut conn, number).unwrap();
                ret_proposal_ids.push(proposal_ids)

            }
            Err(error) => {
                println!("Parsing error: {:?}", error);
            }
        }
    }
    Json(ret_proposal_ids)
}

pub async fn create_dao_handler(
    Query(params): Query<HashMap<String, Vec<String>>>,
) -> Json<String> {
}
