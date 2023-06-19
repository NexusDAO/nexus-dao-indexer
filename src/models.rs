use super::schema::record;
use diesel::prelude::*;
use diesel::sql_types::Array;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = record)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Record {
    pub program: String,
    pub function: String,
    pub inputs: String,
    pub outputs: String,
    pub block_hash: String,
    pub previous_hash: String,
    pub transaction_id: String,
    pub transition_id: String,
    pub network: i64,
    pub height: i64,
    pub timestamp: i64,
}

#[derive(Insertable)]
#[diesel(table_name = record)]
pub struct NewRecord<'a> {
    pub program: &'a str,
    pub function: &'a str,
    pub inputs: &'a str,
    pub outputs: &'a str,
    pub block_hash: &'a str,
    pub previous_hash: &'a str,
    pub transaction_id: &'a str,
    pub transition_id: &'a str,
    pub network: i64,
    pub height: i64,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub r#type: String,
    pub id: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub r#type: String,
    pub id: String,
    pub checksum: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct RespRecords {
    pub records: Vec<String>,
    pub transaction_id: String,
    pub transition_id: String,
    pub network: i64,
    pub height: i64,
    pub timestamp: i64,
}
