use super::schema::record;
use super::schema::profile;
use super::schema::daos_schema;
use super::schema::token_info;
use super::schema::dao;
use super::schema::token;
use super::schema::stake;
use super::schema::proposal;
use super::schema::vote;
use super::schema::voting_results;
use super::schema::token_info_schema;
use super::schema::hold_token;
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
#[diesel(table_name = daos_schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DaosSchema {
    pub name: String,
    pub dao_type: i64,
    pub creater: String,
    pub icon: String,
    pub description: String,
    pub official_link: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = token_info)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenInfo {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub supply: i64,
    pub decimals: i64,
    pub max_mint_amount: i64,
    pub minted_amount: i64,
    pub dao_id: i64,
    pub only_creator_can_mint: bool,
}

#[derive(Insertable)]
#[diesel(table_name = token_info)]
pub struct NewTokenInfo<'a> {
    pub id: i64,
    pub name: &'a str,
    pub symbol: &'a str,
    pub supply: i64,
    pub decimals: i64,
    pub max_mint_amount: i64,
    pub minted_amount: i64,
    pub dao_id: i64,
    pub only_creator_can_mint: bool,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = dao)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dao {
    pub id: i64,
    pub name: String,
    pub dao_type: i64,
    pub creater: String,
    pub token_info_id: i64,
    pub icon: String,
    pub description: String,
    pub official_link: String,
    pub proposal_count: i64,
    pub pass_proposal_count: i64,
    pub vote_count: i64,
    pub passed_votes_proportion: i64,  
    pub passed_tokens_proportion: i64,
}

#[derive(Insertable)]
#[diesel(table_name = dao)]
pub struct NewDao<'a> {
    pub id: i64,
    pub name: &'a str,
    pub dao_type: i64,
    pub creater: &'a str,
    pub token_info_id: i64,
    pub icon: &'a str,
    pub description: &'a str,
    pub official_link: &'a str,
    pub proposal_count: i64,
    pub pass_proposal_count: i64,
    pub vote_count: i64,
    pub passed_votes_proportion: i64,  
    pub passed_tokens_proportion: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub owner: String,
    pub gates: i64,
    pub token_info_id: i64,
    pub amount: i64,
    pub expires: i64,
    pub staked_at: i64, 
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
    pub id: i64,
    pub title: String,
    pub proposer: String,
    pub summary: String,
    pub body: String,
    pub dao_id: i64,
    pub created: i64,
    pub duration: i64,
    pub proposer_type: i64,
    pub adopt: i64,
    pub reject: i64,
    pub status: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = vote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vote {
    pub voter: String,
    pub proposal_id: i64,
    pub token_id: i64,
    pub is_agreed: bool,

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
#[diesel(table_name = token_info_schema)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenInfoSchema {
    pub name: String,
    pub symbol: String,
    pub supply: i64,
    pub decimals: i64,
    pub max_mint_amount: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = hold_token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HoldToken {
    pub address: String,
    pub amount: i64,
    pub token_info_id: i64,
}


// #[derive(Queryable, Selectable, Deserialize, Serialize)]
// #[diesel(table_name = dao_table)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct DaoTable {
//     pub organization_name: String,
//     pub fund_rank: i64,
//     pub total_funds: String,
//     pub token_count: String,
//     pub token_price: String,
//     pub token_name: String,
//     pub token_holder_count: i64,
//     pub token_staker_count: i64,
//     pub proposal_count: i64,
//     pub vote_count: i64,
//     pub proposal_pass_rate: i64,
// }

pub enum DaoType {
    Finance,
    Governance,
    Community,
    Investment,
    Creative,
    Charity,
    Education,
}