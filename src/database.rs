use crate::models::NewRecord;
use crate::proto::Records;
use crate::schema::balances::dsl::balances;
use crate::schema::balances::key;
use crate::{
    models::{
        AutoIncrement, Balances, Daos, ExtendPledgePeriod, Input, NewAutoIncrement, NewBalances,
        NewDaos, NewExtendPledgePeriod, NewProfiles, NewProposals, NewStakeAmounts, NewToken,
        NewTokenInfos, NewVotes, Output, Profiles, Proposals, Record, StakeAmounts, Token,
        TokenInfos, Votes,
    },
    schema::{self},
};
use anyhow::{Error, Ok};
use diesel::{
    r2d2::{ConnectionManager, PoolError},
    BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<PgConnection>> = create_pg_pool().unwrap();
}

pub fn batch_insert_records(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    records: &Records,
) -> Result<(), Error> {
    use schema::record;

    for record in records.records.iter() {
        let inputs = record
            .inputs
            .iter()
            .map(|input| Input {
                r#type: input.r#type.clone(),
                id: input.id.clone(),
                value: input.value.clone(),
                tag: input.tag.clone(),
            })
            .collect::<Vec<Input>>();

        let outputs = record
            .outputs
            .iter()
            .map(|output| Output {
                r#type: output.r#type.clone(),
                id: output.id.clone(),
                checksum: output.checksum.clone(),
                value: output.value.clone(),
            })
            .collect::<Vec<Output>>();

        let new_record = NewRecord {
            program: &record.program,
            function: &record.function,
            inputs: &serde_json::to_string(&inputs).unwrap(),
            outputs: &serde_json::to_string(&outputs).unwrap(),
            block_hash: &record.block_hash,
            previous_hash: &record.previous_hash,
            transaction_id: &record.transaction_id,
            transition_id: &record.transition_id,
            network: record.network as i64,
            height: record.height as i64,
            timestamp: record.timestamp as i64,
        };

        diesel::insert_into(record::table)
            .values(&new_record)
            .on_conflict(record::transition_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
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
) -> Result<Vec<i64>, Error> {
    use schema::daos::dsl::*;
    let dao_ids: Vec<i64> = daos.select(id).load(conn).expect("Error loading records");

    Ok(dao_ids)
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

pub fn get_token_info_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    token_info_id: i64,
) -> Result<TokenInfos, Error> {
    use schema::token_infos::dsl::*;

    let mut ret_token_infos: Vec<TokenInfos> = token_infos
        .filter(id.eq(token_info_id))
        .select(TokenInfos::as_select())
        .load(conn)
        .expect("Error loading token_infos");

    let ret_token_infos_op = ret_token_infos.pop();
    if ret_token_infos_op.is_some() {
        return Ok(ret_token_infos_op.unwrap());
    }
    return Err(Error::msg("The dao was not found"));
}

pub fn get_dao_proposal_ids_by_dao_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_id: i64,
) -> Result<Vec<i64>, Error> {
    use schema::proposals::dsl::*;
    let mut ret_proposal_ids: Vec<i64> = Vec::new();

    let prop: Vec<i64> = proposals
        .filter(dao_id.eq(param_id))
        .select(id)
        .load(conn)
        .expect("The proposal was not found");

    if prop.is_empty() {
        return Err(Error::msg("The proposal was not found"));
    }

    for i in prop {
        ret_proposal_ids.push(i)
    }

    Ok(ret_proposal_ids)
}

pub fn get_balances_by_owner(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_owner: String,
) -> Result<Vec<Balances>, Error> {
    use schema::balances::dsl::*;

    let ret_balances: Vec<Balances> = balances
        .filter(owner.eq(param_owner))
        .select(Balances::as_select())
        .load(conn)
        .expect("Error loading balances");

    Ok(ret_balances)
}

pub fn get_stakes_by_owner(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_owner: String,
) -> Result<Vec<StakeAmounts>, Error> {
    use schema::stake_amounts::dsl::*;

    let ret_stakes: Vec<StakeAmounts> = stake_amounts
        .filter(owner.eq(param_owner))
        .select(StakeAmounts::as_select())
        .load(conn)
        .expect("Error loading balances");

    Ok(ret_stakes)
}

pub fn get_vote_by_key(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_key: String,
) -> Result<Votes, Error> {
    use schema::votes::dsl::*;

    let mut ret_votes: Vec<Votes> = votes
        .filter(key.eq(param_key))
        .select(Votes::as_select())
        .load(conn)
        .expect("Error loading balances");

    if ret_votes.is_empty() {
        return Err(Error::msg("The votes was not found"));
    }

    Ok(ret_votes.pop().unwrap())
}

pub fn get_vote_by_voter(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_voter: String,
) -> Result<Vec<Votes>, Error> {
    use schema::votes::dsl::*;

    let ret_votes: Vec<Votes> = votes
        .filter(voter.eq(param_voter))
        .select(Votes::as_select())
        .load(conn)
        .expect("Error loading votes");

    Ok(ret_votes)
}

pub fn get_auto_increment_by_key(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_key: i64,
) -> Result<AutoIncrement, Error> {
    use schema::auto_increment::dsl::*;

    let mut ret_auto_increment: Vec<AutoIncrement> = auto_increment
        .filter(key.eq(param_key))
        .select(AutoIncrement::as_select())
        .load(conn)
        .expect("Error loading auto_increment");

    if ret_auto_increment.is_empty() {
        return Err(Error::msg("The auto_increment was not found"));
    }

    Ok(ret_auto_increment.pop().unwrap())
}

pub fn get_extend_pledge_period_by_key(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_key: i64,
) -> Result<ExtendPledgePeriod, Error> {
    use schema::extend_pledge_period::dsl::*;

    let mut ret_extend_pledge_period: Vec<ExtendPledgePeriod> = extend_pledge_period
        .filter(key.eq(param_key))
        .select(ExtendPledgePeriod::as_select())
        .load(conn)
        .expect("Error loading extend_pledge_period");

    if ret_extend_pledge_period.is_empty() {
        return Err(Error::msg("The extend_pledge_period was not found"));
    }

    Ok(ret_extend_pledge_period.pop().unwrap())
}

pub fn get_pledgers_by_token_info_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token_info_id: i64,
) -> Result<i64, Error> {
    use schema::stake_amounts::dsl::*;

    let stake: Vec<StakeAmounts> = stake_amounts
        .filter(token_info_id.eq(param_token_info_id))
        .select(StakeAmounts::as_select())
        .distinct_on(owner)
        .load(conn)
        .expect("Error loading stakes");

    Ok(stake.len().to_string().parse::<i64>().unwrap())
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

    Ok(count.to_string())
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

    Ok(count.to_string())
}

pub fn get_creating_dao_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<i64>, Error> {
    use schema::proposals::dsl::*;
    let prop: Vec<i64> = proposals
        .filter(
            duration
                .eq(0)
                .and(status.eq(0).or(status.eq(1)))
                .and(type_.eq(0)),
        )
        .select(id)
        .load(conn)
        .expect("Error loading stakes");

    Ok(prop)
}

pub fn get_proposals_by_proposal_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_id: i64,
) -> Result<Proposals, Error> {
    use schema::proposals::dsl::*;

    let mut prop: Vec<Proposals> = proposals
        .filter(id.eq(param_id))
        .select(Proposals::as_select())
        .load(conn)
        .expect("The proposal was not found");

    if prop.is_empty() {
        return Err(Error::msg("The proposal was not found"));
    }

    let ret_prop = prop.pop().unwrap();

    Ok(ret_prop)
}

pub fn get_all_proposal_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<i64>, Error> {
    use schema::proposals::dsl::*;

    let prop: Vec<i64> = proposals
        .select(id)
        .load(conn)
        .expect("The proposal was not found");

    Ok(prop)
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

pub fn update_token(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token: Token,
) -> Result<String, Error> {
    use schema::token::dsl::*;

    diesel::update(token.filter(owner.eq(param_token.owner)))
        .set((
            gates.eq(param_token.gates),
            token_info_id.eq(param_token.token_info_id),
            amount.eq(param_token.amount),
            expires.eq(param_token.expires),
            staked_at.eq(param_token.staked_at),
        ))
        .execute(conn)
        .expect("Update: Error");

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

pub fn upsert_token_info(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token_info: TokenInfos,
) -> Result<String, Error> {
    use schema::token_infos::dsl::*;
    let param_id = param_token_info.id.clone();

    let get_token_infos: Vec<TokenInfos> = token_infos
        .filter(id.eq(param_id))
        .select(TokenInfos::as_select())
        .load(conn)
        .expect("");

    if get_token_infos.is_empty() {
        insert_token_info(conn, param_token_info)
    } else {
        update_token_info(conn, param_token_info)
    }
}

pub fn update_token_info(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_token_info: TokenInfos,
) -> Result<String, Error> {
    use schema::token_infos::dsl::*;

    diesel::update(token_infos.filter(id.eq(param_token_info.id)))
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
        .execute(conn)
        .expect("Update: Error");

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

pub fn upsert_balances(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_balances: Balances,
) -> Result<String, Error> {
    use schema::balances;
    let param_key = param_balances.key.clone();

    let get_balances: Vec<Balances> = balances
        .filter(key.eq(param_key))
        .select(Balances::as_select())
        .load(conn)
        .expect("");

    if get_balances.is_empty() {
        insert_balances(conn, param_balances)
    } else {
        update_balances(conn, param_balances)
    }
}

pub fn update_balances(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_balances: Balances,
) -> Result<String, Error> {
    use schema::balances::dsl::*;

    diesel::update(balances.filter(key.eq(param_balances.key)))
        .set((
            owner.eq(param_balances.owner),
            amount.eq(param_balances.amount),
            token_info_id.eq(param_balances.token_info_id),
        ))
        .execute(conn)
        .expect("Update: Error");

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

pub fn upsert_stake_amounts(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_stake_amounts: StakeAmounts,
) -> Result<String, Error> {
    use schema::stake_amounts::dsl::*;
    let param_key = param_stake_amounts.key.clone();

    let get_stake_amounts: Vec<StakeAmounts> = stake_amounts
        .filter(key.eq(param_key))
        .select(StakeAmounts::as_select())
        .load(conn)
        .expect("");

    if get_stake_amounts.is_empty() {
        insert_stake_amounts(conn, param_stake_amounts)
    } else {
        update_stake_amounts(conn, param_stake_amounts)
    }
}

pub fn update_stake_amounts(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_stake_amounts: StakeAmounts,
) -> Result<String, Error> {
    use schema::stake_amounts::dsl::*;

    diesel::update(stake_amounts.filter(key.eq(param_stake_amounts.key)))
        .set((
            owner.eq(param_stake_amounts.owner),
            amount.eq(param_stake_amounts.amount),
            token_info_id.eq(param_stake_amounts.token_info_id),
        ))
        .execute(conn)
        .expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn insert_profile(
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

pub fn upsert_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_profile: Profiles,
) -> Result<String, Error> {
    use schema::profiles::dsl::*;
    let addr = param_profile.address.clone();
    let get_profiles: Vec<Profiles> = profiles
        .filter(address.eq(addr))
        .select(Profiles::as_select())
        .load(conn)
        .expect("");

    if get_profiles.is_empty() {
        insert_profile(conn, param_profile)
    } else {
        update_profile(conn, param_profile)
    }
}

pub fn update_profile(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_profile: Profiles,
) -> Result<String, Error> {
    use schema::profiles::dsl::*;

    diesel::update(profiles.filter(address.eq(param_profile.address)))
        .set((
            name.eq(param_profile.name),
            avatar.eq(param_profile.avatar),
            bio.eq(param_profile.bio),
        ))
        .execute(conn)
        .expect("Update: Error");

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
        creator: &param_dao.creator,
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

pub fn upsert_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_dao: Daos,
) -> Result<String, Error> {
    use schema::daos::dsl::*;
    let param_id = param_dao.id.clone();

    let get_daos: Vec<Daos> = daos
        .filter(id.eq(param_id))
        .select(Daos::as_select())
        .load(conn)
        .expect("");

    if get_daos.is_empty() {
        create_dao(conn, param_dao)
    } else {
        update_dao(conn, param_dao)
    }
}

pub fn update_dao(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_dao: Daos,
) -> Result<String, Error> {
    use schema::daos::dsl::*;

    diesel::update(daos.filter(id.eq(param_dao.id)))
        .set((
            name.eq(param_dao.name),
            dao_type.eq(param_dao.dao_type),
            creator.eq(param_dao.creator),
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
        .execute(conn)
        .expect("Update: Error");

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
        type_: param_proposal.type_,
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

pub fn upsert_proposal(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_proposal: Proposals,
) -> Result<String, Error> {
    use schema::proposals::dsl::*;

    let param_id = param_proposal.id.clone();

    let get_proposal: Vec<Proposals> = proposals
        .filter(id.eq(param_id))
        .select(Proposals::as_select())
        .load(conn)
        .expect("");

    if get_proposal.is_empty() {
        create_proposal(conn, param_proposal)
    } else {
        update_proposal(conn, param_proposal)
    }
}

pub fn update_proposal(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_proposal: Proposals,
) -> Result<String, Error> {
    use schema::proposals::dsl::*;

    diesel::update(proposals.filter(id.eq(param_proposal.id)))
        .set((
            title.eq(param_proposal.title),
            proposer.eq(param_proposal.proposer),
            summary.eq(param_proposal.summary),
            body.eq(param_proposal.body),
            dao_id.eq(param_proposal.dao_id),
            created.eq(param_proposal.created),
            duration.eq(param_proposal.duration),
            type_.eq(param_proposal.type_),
            adopt.eq(param_proposal.adopt),
            reject.eq(param_proposal.reject),
            status.eq(param_proposal.status),
        ))
        .execute(conn)
        .expect("Update: Error");

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

pub fn upsert_auto_increment(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_auto_increment: AutoIncrement,
) -> Result<String, Error> {
    use schema::auto_increment::dsl::*;
    let param_key = param_auto_increment.key.clone();

    let get_auto_increment: Vec<AutoIncrement> = auto_increment
        .filter(key.eq(param_key))
        .select(AutoIncrement::as_select())
        .load(conn)
        .expect("");

    if get_auto_increment.is_empty() {
        create_auto_increment(conn, param_auto_increment)
    } else {
        update_auto_increment(conn, param_auto_increment)
    }
}

pub fn update_auto_increment(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_auto_increment: AutoIncrement,
) -> Result<String, Error> {
    use schema::auto_increment::dsl::*;

    diesel::update(auto_increment.filter(key.eq(param_auto_increment.key)))
        .set((value.eq(param_auto_increment.value),))
        .execute(conn)
        .expect("Update: Error");

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

pub fn upsert_extend_pledge_period(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_extend_pledge_period: ExtendPledgePeriod,
) -> Result<String, Error> {
    use schema::extend_pledge_period::dsl::*;
    let param_key = param_extend_pledge_period.key.clone();

    let get_extend_pledge_period: Vec<ExtendPledgePeriod> = extend_pledge_period
        .filter(key.eq(param_key))
        .select(ExtendPledgePeriod::as_select())
        .load(conn)
        .expect("");

    if get_extend_pledge_period.is_empty() {
        create_extend_pledge_period(conn, param_extend_pledge_period)
    } else {
        update_extend_pledge_period(conn, param_extend_pledge_period)
    }
}

pub fn update_extend_pledge_period(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_extend_pledge_period: ExtendPledgePeriod,
) -> Result<String, Error> {
    use schema::extend_pledge_period::dsl::*;

    diesel::update(extend_pledge_period.filter(key.eq(param_extend_pledge_period.key)))
        .set((value.eq(param_extend_pledge_period.value),))
        .execute(conn)
        .expect("Update: Error");

    Ok("Update successfully!".to_string())
}

pub fn insert_votes(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    param_vote: Votes,
) -> Result<String, Error> {
    use schema::votes;

    let new_vote = NewVotes {
        key: &param_vote.key,
        voter: &param_vote.voter,
        proposal_id: param_vote.proposal_id,
        is_agreed: param_vote.is_agreed,
        time: param_vote.time,
        amount: param_vote.amount,
    };

    diesel::insert_into(votes::table)
        .values(&new_vote)
        .on_conflict(votes::key)
        .do_nothing()
        .execute(conn)?;

    Ok("Insert successfully!".to_string())
}
