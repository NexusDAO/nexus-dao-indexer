use super::schema::record;
use super::schema::profile;
use super::schema::dao_info;
use super::schema::token_info;
use super::schema::dao;
use super::schema::token;
use super::schema::stake;
use super::schema::proposal;
use super::schema::vote;
use super::schema::voting_results;
use super::schema::dao_table;
use diesel::{prelude::*, Queryable};
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


#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub address: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
}

#[derive(Insertable)]
#[diesel(table_name = profile)]
pub struct NewProfile<'a> {
    pub address: &'a str,
    pub name: &'a str,
    pub avatar: &'a str,
    pub bio: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct RespProfile {
    pub address: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = dao_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DaoInfo {
    pub name: String,
    pub dao_type: String,
    pub icon: String,
    pub description: String,
    pub official_link: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = token_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub supply: String,
    pub decimals: String,
    pub contract: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = dao)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dao {
    pub id: String,
    pub creater: String,
    pub info: String,
    pub token: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: String,
    pub owner: String,
    pub amount: String,
    pub token_info: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = stake)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Stake {
    pub id: String,
    pub owner: String,
    pub amount: String,
    pub token: String,
    pub created: i64,
    pub duration: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub proposal_type: String,
    pub summary: String,
    pub body: String,
    pub proposer: String,
    pub stake: String,
    pub dao: String,
    pub created: i64,
    pub duration: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = vote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vote {
    pub proposal_id: String,
    pub choice: bool,

}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = voting_results)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VotingResults {
    pub proposal_id: String,
    pub adopt: i64,
    pub reject: i64,
}


#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = dao_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DaoTable {
    pub organization_name: String,
    pub fund_rank: i64,
    pub total_funds: String,
    pub token_count: String,
    pub token_price: String,
    pub token_name: String,
    pub token_holder_count: i64,
    pub token_staker_count: i64,
    pub proposal_count: i64,
    pub vote_count: i64,
    pub proposal_pass_rate: i64,
}

