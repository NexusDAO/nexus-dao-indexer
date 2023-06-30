use anyhow::{Error, Ok};
use clap::builder::Str;
use diesel::{associations::HasTable, r2d2::{ConnectionManager, PoolError}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, BoolExpressionMethods, PgNetExpressionMethods};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;
use futures03::StreamExt;
use r2d2_postgres::postgres::types::ToSql;

use crate::{
    models::{
        Daos, NewDaos, NewProfiles, NewProposals, NewToken, NewTokenInfos,
        Profiles, Proposals, Record, Token, TokenInfos, AutoIncrement, NewAutoIncrement, Balances, NewBalances, StakeAmounts, NewStakeAmounts, ExtendPledgePeriod, NewExtendPledgePeriod,
    },
    schema::{self},
};
use crate::schema::balances::dsl::balances;
use crate::schema::proposals::dsl::proposals;
use crate::schema::stake_amounts::dsl::stake_amounts;

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
    addr: String,
) -> Result<Profiles, Error> {
    use schema::profiles::dsl::*;

    let mut vec_profiles: Vec<Profiles> = profiles
        .filter(address.eq(addr))
        .select(Profiles::as_select())
        .load(conn)
        .expect("Error loading profile");

    let ret_profiles = vec_profiles.pop();
    if ret_profiles.is_some() {
        return Ok(ret_profiles.unwrap());
    }
    return Err(Error::msg("failed find profile"));
}

pub fn get_all_dao_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<String>, Error> {
    use schema::daos::dsl::*;
    let mut ret_dao_ids: Vec<String> = Vec::new();
    let dao_ids: Vec<i64> = daos.select(id).load(conn).expect("Error loading records");

    for i in dao_ids {
        ret_dao_ids.push(i.to_string())
    }
    Ok(ret_dao_ids)
}

pub fn get_dao_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    dao_id: i64,
) -> Result<Daos, Error> {
    use schema::daos::dsl::*;

    let mut ret_dao: Vec<Daos> = daos
        .filter(id.eq(dao_id))
        .select(Daos::as_select())
        .load(conn)
        .expect("Error loading dao");

    let dao_op = ret_dao.pop();
    if dao_op.is_some() {
        return Ok(dao_op.unwrap());
    }
    return Err(Error::msg("The dao was not found"));
}

pub fn get_dao_proposal_ids_by_dao_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_id: i64,
) -> Result<Vec<String>, Error> {
    use schema::proposals::dsl::*;
    let mut ret_proposal_ids: Vec<String> = Vec::new();

    let prop: Vec<Proposals> = proposals
        .filter(dao_id.eq(param_id))
        .select(Proposals::as_select())
        .load(conn)
        .expect("The proposal was not found");

    if prop.is_empty() {
        return Err(Error::msg("The proposal was not found"));
    }

    for i in prop {
        ret_proposal_ids.push(i.id.to_string())
    }

    Ok(ret_proposal_ids)
}

pub fn get_balances(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_address: String,
) -> Result<Vec<Balances>, Error> {
    use schema::balances::dsl::*;

    let ret_balances: Vec<Balances> = balances
        .filter(owner.eq(param_address))
        .select(Balances::as_select())
        .load(conn)
        .expect("Error loading balances");

    Ok(ret_balances)
}

pub fn get_stakes(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_address: String,
) -> Result<Vec<StakeAmounts>, Error> {
    use schema::stake_amounts::dsl::*;

    let ret_stakes: Vec<StakeAmounts> = stake_amounts
        .filter(owner.eq(param_address))
        .select(StakeAmounts::as_select())
        .load(conn)
        .expect("Error loading balances");

    Ok(ret_stakes)
}

pub fn get_pledgers_total(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<String, Error> {
    use schema::stake_amounts::dsl::*;

    let stake: Vec<StakeAmounts> = stake_amounts
        .select(StakeAmounts::as_select())
        .distinct_on(owner)
        .load(conn)
        .expect("Error loading stakes");

    let count = stake.len();
    Ok(count.to_string())
}

pub fn get_stake_funds_total(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<String, Error> {
    use schema::stake_amounts::dsl::*;

    let stake: Vec<StakeAmounts> = stake_amounts
        .select(StakeAmounts::as_select())
        .load(conn)
        .expect("Error loading stakes");

    let mut count: i64 = 0;
    for i in stake {
        count = count + i.amount
    }

    Ok( count.to_string())
}

pub fn get_funds_total(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<String, Error> {
    use schema::balances::dsl::*;

    let stake: Vec<Balances> = balances
        .select(Balances::as_select())
        .load(conn)
        .expect("Error loading stakes");

    let mut count: i64 = 0;
    for i in stake {
        count = count + i.amount
    }

    Ok( count.to_string())
}

pub fn get_creating_dao_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<String>, Error> {
    use schema::proposals::dsl::*;
    let mut ret_prop_id: Vec<String> = Vec::new();
    let prop: Vec<Proposals> = proposals
        .filter(duration.eq(0).and(status.eq(0).or(status.eq(1))))
        .select(Proposals::as_select())
        .load(conn)
        .expect("Error loading stakes");

    for i in prop {
        ret_prop_id.push(i.id.to_string())
    }
    Ok( ret_prop_id)
}


fn create_pg_pool() -> Result<PgPool, PoolError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    PgPool::builder().build(manager)
}

pub fn insert_token(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token: Token,
) -> Result<String, Error> {
    use schema::token;

    let new_token = NewToken {
        owner: &param_token.owner,
        gates: param_token.gates,
        token_info_id: param_token.token_info_id,
        amount: param_token.amount,
        expires: param_token.expires,
        staked_at: param_token.staked_at,
    };

    diesel::insert_into(token::table)
        .values(&new_token)
        .on_conflict(token::owner)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_token_by_owner(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token: Token,
    param_owner: String,
) -> Result<String, Error> {
    use schema::token::dsl::*;

    diesel::update(token.filter(owner.eq(param_owner)))
        .set((
            gates.eq(param_token.gates),
            token_info_id.eq(param_token.token_info_id),
            amount.eq(param_token.amount),
            expires.eq(param_token.expires),
            staked_at.eq(param_token.staked_at),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn insert_token_info(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token_info: TokenInfos,
) -> Result<String, Error> {
    use schema::token_infos;

    let new_token_info = NewTokenInfos {
        id: param_token_info.id,
        name: &param_token_info.name,
        symbol: &param_token_info.symbol,
        supply: param_token_info.supply,
        decimals: param_token_info.decimals,
        max_mint_amount: param_token_info.max_mint_amount,
        minted_amount: param_token_info.minted_amount,
        dao_id: param_token_info.dao_id,
        only_creator_can_mint: param_token_info.only_creator_can_mint,
    };

    diesel::insert_into(token_infos::table)
        .values(&new_token_info)
        .on_conflict(token_infos::id)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_token_info_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token_info: TokenInfos,
    param_id: i64,
) -> Result<String, Error> {
    use schema::token_infos::dsl::*;

    diesel::update(token_infos.filter(id.eq(param_id)))
        .set((
            name.eq(&param_token_info.name),
            symbol.eq(&param_token_info.symbol),
            supply.eq(param_token_info.supply),
            decimals.eq(param_token_info.decimals),
            max_mint_amount.eq(param_token_info.max_mint_amount),
            minted_amount.eq(param_token_info.minted_amount),
            dao_id.eq(param_token_info.dao_id),
            only_creator_can_mint.eq(param_token_info.only_creator_can_mint),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn insert_balances(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_balances: Balances,
) -> Result<String, Error> {
    use schema::balances;

    let new_balances = NewBalances {
        key: &param_balances.key,
        owner: &param_balances.owner,
        amount: param_balances.amount,
        token_info_id: param_balances.token_info_id,
    };

    diesel::insert_into(balances::table)
        .values(&new_balances)
        .on_conflict(balances::key)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_balances_by_key(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_balances: Balances,
    param_key: String,
) -> Result<String, Error> {
    use schema::balances::dsl::*;

    diesel::update(balances.filter(key.eq(param_key)))
        .set((
            owner.eq(param_balances.owner),
            amount.eq(param_balances.amount),
            token_info_id.eq(param_balances.token_info_id),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn insert_stake_amounts(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_stake_amounts: StakeAmounts,
) -> Result<String, Error> {
    use schema::stake_amounts;

    let new_stake_amounts = NewStakeAmounts {
        key: &param_stake_amounts.key,
        owner: &param_stake_amounts.owner,
        amount: param_stake_amounts.amount,
        token_info_id: param_stake_amounts.token_info_id,
    };

    diesel::insert_into(stake_amounts::table)
        .values(&new_stake_amounts)
        .on_conflict(stake_amounts::key)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_stake_amounts_by_key(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_stake_amounts: StakeAmounts,
    param_key: String,
) -> Result<String, Error> {
    use schema::stake_amounts::dsl::*;

    diesel::update(stake_amounts.filter(key.eq(param_key)))
        .set((
            owner.eq(param_stake_amounts.owner),
            amount.eq(param_stake_amounts.amount),
            token_info_id.eq(param_stake_amounts.token_info_id),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}


pub fn create_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_profile: Profiles,
) -> Result<String, Error> {
    use schema::profiles;

    let new_profile: NewProfiles<'_> = NewProfiles {
        address: &param_profile.address,
        name: &param_profile.name,
        avatar: &param_profile.avatar,
        bio: &param_profile.bio,
    };

    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .on_conflict(profiles::address)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_profile: Profiles,
    param_addr: String,
) -> Result<String, Error> {
    use schema::profiles::dsl::*;

    diesel::update(profiles.filter(address.eq(param_addr)))
        .set((
            name.eq(param_profile.name),
            avatar.eq(param_profile.avatar),
            bio.eq(param_profile.bio),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn create_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_dao: Daos,
) -> Result<String, Error> {
    use schema::daos;

    let new_dao = NewDaos {
        id: param_dao.id,
        name: &param_dao.name,
        dao_type: param_dao.dao_type,
        creater: &param_dao.creater,
        token_info_id: param_dao.token_info_id,
        icon: &param_dao.icon,
        description: &param_dao.description,
        official_link: &param_dao.official_link,
        proposal_count: param_dao.proposal_count,
        pass_proposal_count: param_dao.pass_proposal_count,
        vote_count: param_dao.vote_count,
        passed_votes_proportion: param_dao.passed_votes_proportion,
        passed_tokens_proportion: param_dao.passed_tokens_proportion,
    };

    diesel::insert_into(daos::table)
        .values(&new_dao)
        .on_conflict(daos::id)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_dao_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_dao: Daos,
    param_id: i64,
) -> Result<String, Error> {
    use schema::daos::dsl::*;

    diesel::update(daos.filter(id.eq(param_id)))
        .set((
            name.eq(param_dao.name),
            dao_type.eq(param_dao.dao_type),
            creater.eq(param_dao.creater),
            token_info_id.eq(param_dao.token_info_id),
            icon.eq(param_dao.icon),
            description.eq(param_dao.description),
            official_link.eq(param_dao.official_link),
            proposal_count.eq(param_dao.proposal_count),
            pass_proposal_count.eq(param_dao.pass_proposal_count),
            vote_count.eq(param_dao.vote_count),
            passed_votes_proportion.eq(param_dao.passed_votes_proportion),
            passed_tokens_proportion.eq(param_dao.passed_tokens_proportion),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn create_proposal(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_proposal: Proposals,
) -> Result<String, Error> {
    use schema::proposals;

    let new_proposal = NewProposals {
        id: param_proposal.id,
        title: &param_proposal.title,
        proposer: &param_proposal.proposer,
        summary: &param_proposal.summary,
        body: &param_proposal.body,
        dao_id: param_proposal.dao_id,
        created: param_proposal.created,
        duration: param_proposal.duration,
        proposer_type: param_proposal.proposer_type,
        adopt: param_proposal.adopt,
        reject: param_proposal.reject,
        status: param_proposal.status,
    };

    diesel::insert_into(proposals::table)
        .values(&new_proposal)
        .on_conflict(proposals::id)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_proposal_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_proposal: Proposals,
    param_id: i64,
) -> Result<String, Error> {
    use schema::proposals::dsl::*;

    diesel::update(proposals.filter(id.eq(param_id)))
        .set((
            title.eq(param_proposal.title),
            proposer.eq(param_proposal.proposer),
            summary.eq(param_proposal.summary),
            body.eq(param_proposal.body),
            dao_id.eq(param_proposal.dao_id),
            created.eq(param_proposal.created),
            duration.eq(param_proposal.duration),
            proposer_type.eq(param_proposal.proposer_type),
            adopt.eq(param_proposal.adopt),
            reject.eq(param_proposal.reject),
            status.eq(param_proposal.status),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn create_auto_increment(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_auto_increment: AutoIncrement,
) -> Result<String, Error> {
    use schema::auto_increment;

    let new_auto_increment = NewAutoIncrement {
        key: param_auto_increment.key,
        value: param_auto_increment.value,
    };

    diesel::insert_into(auto_increment::table)
        .values(&new_auto_increment)
        .on_conflict(auto_increment::key)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_auto_increment(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_auto_increment: AutoIncrement,
    param_key: i64,
) -> Result<String, Error> {
    use schema::auto_increment::dsl::*;

    diesel::update(auto_increment.filter(key.eq(param_key)))
        .set((
            value.eq(param_auto_increment.value),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn create_extend_pledge_period(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_extend_pledge_period: ExtendPledgePeriod,
) -> Result<String, Error> {
    use schema::extend_pledge_period;

    let new_extend_pledge_period = NewExtendPledgePeriod {
        key: param_extend_pledge_period.key,
        value: param_extend_pledge_period.value,
    };

    diesel::insert_into(extend_pledge_period::table)
        .values(&new_extend_pledge_period)
        .on_conflict(extend_pledge_period::key)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}

pub fn update_extend_pledge_period(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_extend_pledge_period: ExtendPledgePeriod,
    param_key: i64,
) -> Result<String, Error> {
    use schema::extend_pledge_period::dsl::*;

    diesel::update(extend_pledge_period.filter(key.eq(param_key)))
        .set((
            value.eq(param_extend_pledge_period.value),
        ))
        .execute(conn).expect("Update: Error");

    Ok("Update successfully!".to_string())
}