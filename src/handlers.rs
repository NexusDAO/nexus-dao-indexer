use std::{collections::HashMap, str::FromStr};
use axum::{extract::Query, response::Json};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use crate::{
    database::{get_records_by_height, POOL, get_profile_by_address, create_profile},
    models::{Output, RespRecords, RespProfile, Profile}, schema::profile::avatar,
};

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


pub async fn get_profile_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<Profile> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let default_addr = "0".to_string();
    let address = params.get("address").unwrap_or(&default_addr);

    let profiles = get_profile_by_address(&mut conn, address).unwrap();
    
    Json(profiles)
}

pub async fn create_profile_handler(
    Query(params): Query<HashMap<String, String>>,
) -> Json<String> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    
    let default_addr = "0".to_string();
    let default_name = "0".to_string();
    let default_avatar = "0".to_string();
    let default_bio = "0".to_string();

    let addr = params.get("address").unwrap_or(&default_addr);
    let names = params.get("name").unwrap_or(&default_name);
    let avatars = params.get("avatar").unwrap_or(&default_avatar);
    let bios = params.get("bio").unwrap_or(&default_bio);
    
    let a = create_profile(&mut conn, addr, names, avatars, bios).unwrap();
    Json("SUCCESS".to_string())
}