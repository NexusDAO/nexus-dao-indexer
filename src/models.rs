use crate::db::get_conn;
use crate::schema::ratify;
use anyhow::Error;
use async_graphql::SimpleObject;
use diesel::{prelude::*, Queryable};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Queryable, Selectable, Deserialize, Serialize)]
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

#[derive(SimpleObject)]
pub struct Mapping {
    pub key: String,
    pub value: String,
}

impl Mapping {
    pub fn find(program_name: &str, mapping_name: &str, mapping_key: &str) -> Result<Self, Error> {
        Ok(Self {
            key: "i am key".to_string(),
            value: "i am value".to_string(),
        })
    }
}

impl Ratify {
    pub fn list_by_height(height: i64) -> Result<Vec<Self>, Error> {
        let mut conn = get_conn()?;

        let result = ratify::dsl::ratify
            .filter(ratify::dsl::height.eq(height))
            .select(Ratify::as_select())
            .load(&mut conn)?;

        Ok(result)
    }
}

impl<'a> NewRatify<'a> {
    pub fn insert(new_ratify: &Self) -> Result<(), Error> {
        let mut conn = get_conn()?;

        diesel::insert_into(ratify::table)
            .values(new_ratify)
            .execute(&mut conn)?;

        Ok(())
    }
}
