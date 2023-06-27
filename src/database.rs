use anyhow::Error;
use clap::builder::Str;
use diesel::{
    associations::HasTable,
    r2d2::{ConnectionManager, PoolError},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;

use crate::{
    models::{Dao, NewProfile, Profile, Proposal, Record, NewTokenInfo, NewDao, Dao, TokenInfo},
    schema,
};

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
    addr: &String,
    names: &String,
    avatars: &String,
    bios: &String,
) -> Result<String, Error> {
    use schema::profile;

    let new_profile: NewProfile<'_> = NewProfile {
        address: addr,
        name: names,
        avatar: avatars,
        bio: bios,
    };

    diesel::insert_into(profile::table)
        .values(&new_profile)
        .on_conflict(profile::address)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    addr: &String,
    name: &String,
    avatar: &String,
    bio: &String,
) -> Result<String, Error> {
    use schema::profile::dsl::*;

    diesel::update(profile.filter(address.eq(addr)))
        .set((name.eq(name), avatar.eq(avatar), bio.eq(bio)))
        .execute(conn);

    Ok("Update successfully!".to_string())
}

pub fn get_all_dao_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<String>, Error> {
    use schema::dao::dsl::*;
    let mut ret_dao_ids: Vec<String> = Vec::new();
    let dao_ids: Vec<i64> = dao.select(id).load(conn).expect("Error loading records");

    for i in dao_ids {
        ret_dao_ids.push(i.to_string())
    }
    Ok(ret_dao_ids)
}

pub fn get_dao_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    dao_id: i64,
) -> Result<Dao, Error> {
    use schema::dao::dsl::*;

    let daos: Vec<Dao> = dao
        .filter(id.eq(dao_id))
        .select(Dao::as_select())
        .load(conn)
        .expect("Error loading dao");

    Ok(daos[0])
}

pub fn get_dao_proposal_ids_by_dao_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    dao_id: i64,
) -> Result<Vec<String>, Error> {
    use schema::proposal::dsl::*;
    let mut ret_proposal_ids: Vec<String> = Vec::new();

    let proposals: Vec<Proposal> = proposal
        .filter(dao_id.eq(dao_id))
        .select(Proposal::as_select())
        .load(conn)
        .expect("Error loading daos");

    for i in proposals {
        ret_proposal_ids.push(i.id.to_string())
    }

    Ok(ret_proposal_ids)
}

pub fn create_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    daos: Dao, token_infos: TokenInfo, proposal_id: i64
) -> Result<String, Error> {
    use schema::proposal::dsl::*;

    let proposals:Vec<Proposal> = proposal
        .filter(id.eq(proposal_id))
        .select(Proposal::as_select())
        .load(conn)
        .expect("Error loading proposal");

    assert_eq!(proposals[0].proposer, daos.creater);
    assert_eq!(proposals[0].dao_id, 0);
    assert_eq!(proposals[0].status, 2);
    assert_eq!(proposals[0].proposer_type, 0);
    assert!(daos.passed_votes_proportion<=100);
    assert!(daos.passed_tokens_proportion<=100);

    use schema::dao;

    let new_dao = NewDao {
        id: daos.id,
        name: &daos.name,
        dao_type: daos.dao_type,
        creater: &daos.creater,
        token_info_id: daos.token_info_id,
        icon: &daos.icon,
        description: &daos.description,
        official_link: &daos.official_link,
        proposal_count: daos.proposal_count,
        pass_proposal_count: daos.pass_proposal_count,
        vote_count: daos.vote_count,
        passed_votes_proportion: daos.passed_votes_proportion,  
        passed_tokens_proportion: daos.passed_tokens_proportion,
    };

    diesel::insert_into(dao::table)
        .values(&new_dao)
        .on_conflict(dao::id)
        .do_nothing()
        .execute(conn)?;
    
    use schema::token_info;

    let new_token_info = NewTokenInfo {
        id: token_infos.id,
        name: &token_infos.name,
        symbol: &token_infos.symbol,
        supply: token_infos.supply,
        decimals: token_infos.decimals,
        max_mint_amount: token_infos.max_mint_amount,
        minted_amount: token_infos.minted_amount,
        dao_id: token_infos.dao_id,
        only_creator_can_mint: token_infos.only_creator_can_mint,
    };

    diesel::insert_into(token_info::table)
        .values(&new_token_info)
        .on_conflict(token_info::id)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

// pub fn update_dao(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
// ) -> Result<String, Error> {
// }

// pub fn get_token_info(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
// ) -> Result<Dao, Error> {
// }

// pub fn get_balances(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

// pub fn get_stakes(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

// pub fn use_get_pledgers_total(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

// pub fn get_stake_funds_total(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

// pub fn transfer(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

// pub fn stake_token(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

// pub fn claim_token(
//     conn: &mut PooledConnection<ConnectionManager<PgConnection>>,

// ) -> Result<Dao, Error> {

// }

fn create_pg_pool() -> Result<PgPool, PoolError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    PgPool::builder().build(manager)
}
