use super::schema::auto_increment;
use super::schema::balances;
use super::schema::daos;
use super::schema::extend_pledge_period;
use super::schema::profiles;
use super::schema::proposals;
use super::schema::record;
use super::schema::stake_amounts;
use super::schema::token;
use super::schema::token_infos;
use super::schema::votes;
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
    pub tag: String,
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
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profiles {
    pub address: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
}

#[derive(Insertable)]
#[diesel(table_name = profiles)]
pub struct NewProfiles<'a> {
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

// #[derive(Queryable, Selectable, Deserialize, Serialize)]
// #[diesel(table_name = daos_schema)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct DaosSchema {
//     pub name: String,
//     pub dao_type: i64,
//     pub creator: String,
//     pub icon: String,
//     pub description: String,
//     pub official_link: String,
// }

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = token_infos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenInfos {
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
#[diesel(table_name = token_infos)]
pub struct NewTokenInfos<'a> {
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
#[diesel(table_name = daos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Daos {
    pub id: i64,
    pub name: String,
    pub dao_type: i64,
    pub creator: String,
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
#[diesel(table_name = daos)]
pub struct NewDaos<'a> {
    pub id: i64,
    pub name: &'a str,
    pub dao_type: i64,
    pub creator: &'a str,
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

#[derive(Insertable)]
#[diesel(table_name = token)]
pub struct NewToken<'a> {
    pub owner: &'a str,
    pub gates: i64,
    pub token_info_id: i64,
    pub amount: i64,
    pub expires: i64,
    pub staked_at: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Proposals {
    pub id: i64,
    pub title: String,
    pub proposer: String,
    pub summary: String,
    pub body: String,
    pub dao_id: i64,
    pub created: i64,
    pub duration: i64,
    pub type_: i64,
    pub adopt: i64,
    pub reject: i64,
    pub status: i64,
}

#[derive(Insertable)]
#[diesel(table_name = proposals)]
pub struct NewProposals<'a> {
    pub id: i64,
    pub title: &'a str,
    pub proposer: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
    pub dao_id: i64,
    pub created: i64,
    pub duration: i64,
    pub type_: i64,
    pub adopt: i64,
    pub reject: i64,
    pub status: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Votes {
    pub key: String,
    pub voter: String,
    pub proposal_id: i64,
    pub is_agreed: bool,
    pub time: i64,
    pub amount: i64,
}

#[derive(Insertable)]
#[diesel(table_name = votes)]
pub struct NewVotes<'a> {
    pub key: &'a str,
    pub voter: &'a str,
    pub proposal_id: i64,
    pub is_agreed: bool,
    pub time: i64,
    pub amount: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = auto_increment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AutoIncrement {
    pub key: i64,
    pub value: i64,
}

#[derive(Insertable)]
#[diesel(table_name = auto_increment)]
pub struct NewAutoIncrement {
    pub key: i64,
    pub value: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Balances {
    pub key: String,
    pub owner: String,
    pub amount: i64,
    pub token_info_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = balances)]
pub struct NewBalances<'a> {
    pub key: &'a str,
    pub owner: &'a str,
    pub amount: i64,
    pub token_info_id: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = stake_amounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StakeAmounts {
    pub key: String,
    pub owner: String,
    pub amount: i64,
    pub token_info_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = stake_amounts)]
pub struct NewStakeAmounts<'a> {
    pub key: &'a str,
    pub owner: &'a str,
    pub amount: i64,
    pub token_info_id: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = extend_pledge_period)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExtendPledgePeriod {
    pub key: i64,
    pub value: i64,
}

#[derive(Insertable)]
#[diesel(table_name = extend_pledge_period)]
pub struct NewExtendPledgePeriod {
    pub key: i64,
    pub value: i64,
}
