use anyhow::Error;
use clap::builder::Str;
use diesel::{
    r2d2::{ConnectionManager, PoolError},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, associations::HasTable,
};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;

use crate::{models::{Record, Profile, NewProfile, Dao}, schema};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<PgConnection>> = create_pg_pool().unwrap();
}

pub fn get_records_by_height(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    start_block: i64,
    end_block: i64,
) -> Result<Vec<Record>, Error> {
    use schema::record::dsl::*;

    let records = record
        .filter(height.between(start_block, end_block))
        .select(Record::as_select())
        .load(conn)
        .expect("Error loading records");

    Ok(records)
}



pub fn get_profile_by_address(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    addr: &String,
) -> Result<Profile, Error> {
    use schema::profile::dsl::*;

    let profiles = profile
        .filter(address.eq(addr))
        .select(Profile::as_select())
        .load(conn)
        .expect("Error loading records");
        
    Ok(profiles[0])
}

pub fn create_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    addr: &String, names: &String, avatars: &String, bios: &String
) -> Result<String,Error> {
    use schema::profile;

    let new_profile: NewProfile<'_> = NewProfile { address: addr, name: names, avatar: avatars, bio: bios };
    
    diesel::insert_into(profile::table)
    .values(&new_profile)
    .on_conflict(profile::address)
    .do_nothing()
    .execute(conn)?;
        
    Ok("Insert successfully!".to_string())
}

pub fn update_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    addr: &str, name: &str, avatar: &str, bio: &str
) -> Result<String, Error> {
    use schema::profile::dsl::*;

    let profiles = diesel::update(profile.filter(address.eq(addr)))
        .set((
            name.eq(name),
            avatar.eq(avatar),
            bio.eq(bio),
        ))
        .execute(conn)
        .unwrap();

    Ok("Insert successfully!".to_string())
}

pub fn get_all_dao_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<String>, Error> {
    use schema::dao::dsl::*;

    let dao_ids: Vec<String> = dao
        .select(id)
        .load(conn)
        .expect("Error loading records");
    
    Ok(dao_ids)
}

pub fn get_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    dao_id: &str
) -> Result<Dao, Error> {
    use schema::dao::dsl::*;

    let daos = dao
        .filter(id.eq(dao_id))
        .select(Record::as_select())
        .load(conn)
        .expect("Error loading daos");

    Ok(daos)
}

pub fn get_governance_token_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    dao_id: String
) -> String {
    use schema::dao::dsl::*;

    let daos = dao
        .filter(id.eq(dao_id))
        .select(Record::as_select())
        .load(conn)
        .expect("Error loading daos");
    for dao in daos {
        
    }
    return  "0".to_string();
}

pub fn get_dao_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_dao_open_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_dao_adopted_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_dao_rejected_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn create_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn update_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_token_info(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_stakes_count(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_holders_count(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_balances(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn get_stakes(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn transfer(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn stake_token(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

pub fn claim_token(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

) -> Result<Dao, Error> {
 
}

fn create_pg_pool() -> Result<PgPool, PoolError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    PgPool::builder().build(manager)
}
