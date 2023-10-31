use crate::schema::ratify;
use diesel::{prelude::*, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = ratify)]
pub struct Ratify {
    pub ratification_id: String,
    pub height: i64,
    pub type_: String,
    pub starting_round: Option<String>,
    pub total_stake: Option<String>,
    pub block_reward: Option<String>,
    pub puzzle_reward: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = ratify)]
pub struct NewRatify<'a> {
    pub ratification_id: &'a str,
    pub height: i64,
    pub type_: &'a str,
    pub starting_round: Option<&'a str>,
    pub total_stake: Option<&'a str>,
    pub block_reward: Option<&'a str>,
    pub puzzle_reward: Option<&'a str>,
}
