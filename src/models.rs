use crate::db::get_conn;
use crate::schema::{mapping, operation, ratify};
use anyhow::Result;
use async_graphql::SimpleObject;
use diesel::{prelude::*, NotFound, Queryable};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = mapping)]
pub struct Mapping {
    pub key_id: String,
    pub value_id: String,
    pub mapping_id: String,
    pub key: String,
    pub value: String,
    pub mapping_name: String,
    pub program_name: String,
    pub removed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = mapping)]
pub struct NewMapping<'a> {
    pub key_id: Option<&'a str>,
    pub value_id: Option<&'a str>,
    pub mapping_id: &'a str,
    pub key: Option<&'a str>,
    pub value: Option<&'a str>,
    pub mapping_name: &'a str,
    pub program_name: &'a str,
    pub removed: bool,
}

#[derive(SimpleObject, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = operation)]
pub struct Operation {
    pub id: i32,
    pub type_: String,
    pub program_name: String,
    pub mapping_id: String,
    pub key_id: Option<String>,
    pub value_id: Option<String>,
    pub mapping_name: String,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = operation)]
pub struct NewOperation<'a> {
    pub type_: &'a str,
    pub program_name: &'a str,
    pub mapping_id: &'a str,
    pub key_id: Option<&'a str>,
    pub value_id: Option<&'a str>,
    pub mapping_name: &'a str,
    pub key: Option<&'a str>,
    pub value: Option<&'a str>,
}

#[derive(SimpleObject, Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = ratify)]
pub struct Ratify {
    pub id: i32,
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

impl Mapping {
    pub fn get_mapping_by_mapping_key(
        program_name: &str,
        mapping_name: &str,
        mapping_key: &str,
    ) -> Result<Self> {
        let conn = &mut get_conn()?;

        let result = mapping::table
            .filter(
                mapping::program_name
                    .eq(program_name)
                    .and(mapping::mapping_name.eq(mapping_name))
                    .and(mapping::key.eq(mapping_key))
                    .and(mapping::removed.eq(false)),
            )
            .select(Mapping::as_select())
            .first(conn)?;

        Ok(result)
    }

    pub fn get_mapping_by_key_id(key_id: &str) -> Result<Self> {
        let conn = &mut get_conn()?;

        let result = mapping::table
            .filter(mapping::key_id.eq(key_id).and(mapping::removed.eq(false)))
            .select(Mapping::as_select())
            .first(conn)?;

        Ok(result)
    }

    pub fn list_mapping_by_program(program_name: &str) -> Result<Vec<Self>> {
        let conn = &mut get_conn()?;

        let result = mapping::table
            .filter(
                mapping::program_name
                    .eq(program_name)
                    .and(mapping::removed.eq(false)),
            )
            .select(Mapping::as_select())
            .load(conn)?;

        Ok(result)
    }

    pub fn remove_key_value(key_id: &str) -> Result<()> {
        let conn = &mut get_conn()?;

        diesel::update(mapping::table.filter(mapping::key_id.eq(key_id)))
            .set(mapping::removed.eq(true))
            .execute(conn)?;

        Ok(())
    }

    pub fn remove_mapping(mapping_name: &str) -> Result<()> {
        let conn = &mut get_conn()?;

        diesel::update(mapping::table.filter(mapping::mapping_name.eq(mapping_name)))
            .set(mapping::removed.eq(true))
            .execute(conn)?;

        Ok(())
    }
}

impl<'a> NewMapping<'a> {
    pub fn upsert(&self) -> Result<()> {
        let conn = &mut get_conn()?;

        match Mapping::get_mapping_by_key_id(self.key_id.unwrap()) {
            Ok(m) => {
                diesel::update(mapping::table.filter(mapping::key_id.eq(m.key_id)))
                    .set((mapping::value.eq(m.value), mapping::value_id.eq(m.value_id)))
                    .execute(conn)?;
            }
            Err(e) => {
                if e.to_string() == NotFound.to_string() {
                    diesel::insert_into(mapping::table)
                        .values(self)
                        .execute(conn)?;
                } else {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

impl Operation {
    pub fn list_by_program_name(program_name: &str) -> Result<Vec<Self>> {
        let conn = &mut get_conn()?;

        let result = operation::table
            .filter(operation::program_name.eq(program_name))
            .select(Operation::as_select())
            .load(conn)?;

        Ok(result)
    }
}

impl<'a> NewOperation<'a> {
    pub fn insert(&self) -> Result<()> {
        let conn = &mut get_conn()?;

        diesel::insert_into(operation::table)
            .values(self)
            .execute(conn)?;

        Ok(())
    }
}

impl Ratify {
    pub fn list_by_height(height: i64) -> Result<Vec<Self>> {
        let conn = &mut get_conn()?;

        let result = ratify::table
            .filter(ratify::height.eq(height))
            .select(Ratify::as_select())
            .load(conn)?;

        Ok(result)
    }
}

impl<'a> NewRatify<'a> {
    pub fn insert(&self) -> Result<()> {
        let conn = &mut get_conn()?;

        diesel::insert_into(ratify::table)
            .values(self)
            .execute(conn)?;

        Ok(())
    }
}
