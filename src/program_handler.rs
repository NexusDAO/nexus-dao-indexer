use crate::{
    database::{
        create_dao, create_extend_pledge_period, create_proposal, insert_token_info, insert_votes,
        update_dao, update_proposal, update_stake_amounts, update_token_info,
        upsert_auto_increment, upsert_balances, upsert_profile, upsert_stake_amounts,
    },
    mapping_struct::{Dao, HoldToken, Profile, Proposal, TokenInfo, Vote},
    models,
    proto::Records,
};
use anyhow::Error;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use snarkvm::{prelude::*, utilities::ToBits};

type CurrentNetwork = snarkvm::prelude::Testnet3;

const PROGRAM_ID: &str = "pledge_vote.aleo";
const AUTO_INCREMENT_TIMESTAMP: u8 = 0;
const AUTO_INCREMENT_TOKEN_INFOS: u8 = 1;
const AUTO_INCREMENT_PROPOSALS: u8 = 2;
const AUTO_INCREMENT_DAOS: u8 = 3;
const AUTO_INCREMENT_VOTES: u8 = 4;
const MAPPING_NAME_AUTO_INCREMENT: &str = "auto_increment";
const MAPPING_NAME_PROFILES: &str = "profiles";
const MAPPING_NAME_DAOS: &str = "daos";
const MAPPING_NAME_TOKEN_INFOS: &str = "token_infos";
const MAPPING_NAME_BALANCES: &str = "balances";
const MAPPING_NAME_STAKE_AMOUNTS: &str = "stake_amounts";
const MAPPING_NAME_PROPOSALS: &str = "proposals";
const MAPPING_NAME_VOTES: &str = "votes";
const MAPPING_NAME_EXTEND_PLEDGE_PERIOD: &str = "extend_pledge_period";

fn bhp_256_hash(data: &String) -> Result<Field<CurrentNetwork>, Error> {
    <CurrentNetwork as Network>::hash_bhp256(&data.to_bits_le())
}

fn fetch_mapping<T: for<'de> Deserialize<'de>>(
    rest_api: &String,
    program_id: &String,
    mapping_name: &String,
    mapping_key: &String,
) -> Result<T, Error> {
    let response: T = ureq::get(&format!(
        "{rest_api}/testnet3/program/{program_id}/mapping/{mapping_name}/{mapping_key}"
    ))
    .call()?
    .into_json()?;

    Ok(response)
}

pub fn program_handler(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    rest_api: &String,
    records: &Records,
) {
    for record in records.records.iter() {
        if record.program != PROGRAM_ID {
            continue;
        };

        match record.function.as_str() {
            "mint" => {
                let owner = &record.finalize[0];
                let hash_owner = bhp_256_hash(owner).unwrap();
                let token_info_id = &record.finalize[2];
                let hash_id = bhp_256_hash(token_info_id).unwrap();

                let output_token_record_ciphertext = &record.outputs[0].value;
                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = &hash_owner.add(hash_id).to_string();

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    token_infos_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_token_info(
                    conn,
                    models::TokenInfos {
                        id: token_info.id as i64,
                        name: token_info.name,
                        symbol: token_info.symbol,
                        supply: token_info.supply as i64,
                        decimals: token_info.decimals as i64,
                        max_mint_amount: token_info.max_mint_amount as i64,
                        minted_amount: token_info.minted_amount as i64,
                        dao_id: token_info.dao_id as i64,
                        only_creator_can_mint: token_info.only_creator_can_mint,
                    },
                );

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_BALANCES.to_string(),
                    balances_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_balances(
                    conn,
                    models::Balances {
                        key: balances_mapping_key.clone(),
                        owner: hold_token.token_owner,
                        amount: hold_token.amount as i64,
                        token_info_id: hold_token.token_info_id as i64,
                    },
                );
            }

            "stake" => {
                let hash_owner: Field<CurrentNetwork> =
                    Field::from_str(&record.finalize[0]).unwrap();
                let token_info_id = &record.finalize[2];
                let hash_id = bhp_256_hash(token_info_id).unwrap();

                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token1_record_ciphertext = &record.outputs[0].value;
                let output_token2_record_ciphertext = &record.outputs[1].value;
                let stake_amounts_mapping_key = &hash_owner.add(hash_id).to_string();

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_STAKE_AMOUNTS.to_string(),
                    stake_amounts_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_stake_amounts(
                    conn,
                    models::StakeAmounts {
                        key: stake_amounts_mapping_key.clone(),
                        owner: hold_token.token_owner,
                        amount: hold_token.amount as i64,
                        token_info_id: hold_token.token_info_id as i64,
                    },
                );
            }

            "unstake" => {
                let hash_owner: Field<CurrentNetwork> =
                    Field::from_str(&record.finalize[1]).unwrap();
                let token_info_id = &record.finalize[3];
                let hash_id = bhp_256_hash(&token_info_id).unwrap();

                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let stake_amounts_mapping_key = &hash_owner.add(hash_id).to_string();

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_STAKE_AMOUNTS.to_string(),
                    stake_amounts_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_stake_amounts(
                    conn,
                    models::StakeAmounts {
                        key: stake_amounts_mapping_key.clone(),
                        owner: hold_token.token_owner,
                        amount: hold_token.amount as i64,
                        token_info_id: hold_token.token_info_id as i64,
                    },
                );
            }

            "transfer" => {
                let sender = &record.finalize[0];
                let receiver = &record.finalize[1];
                let token_info_id = &record.finalize[3];

                let hash_id = bhp_256_hash(&token_info_id).unwrap();
                let sender_hash = bhp_256_hash(&sender).unwrap();
                let receiver_hash = bhp_256_hash(&receiver).unwrap();

                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token1_record_ciphertext = &record.outputs[0].value;
                let output_token2_record_ciphertext = &record.outputs[1].value;
                let sender_balances_mapping_key = &sender_hash.add(hash_id).to_string();
                let receiver_balances_mapping_key = &receiver_hash.add(hash_id).to_string();

                let sender_hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_BALANCES.to_string(),
                    sender_balances_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_balances(
                    conn,
                    models::Balances {
                        key: sender_balances_mapping_key.clone(),
                        owner: sender_hold_token.token_owner,
                        amount: sender_hold_token.amount as i64,
                        token_info_id: sender_hold_token.token_info_id as i64,
                    },
                );

                let receiver_hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_BALANCES.to_string(),
                    receiver_balances_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_balances(
                    conn,
                    models::Balances {
                        key: receiver_balances_mapping_key.clone(),
                        owner: receiver_hold_token.token_owner,
                        amount: receiver_hold_token.amount as i64,
                        token_info_id: receiver_hold_token.token_info_id as i64,
                    },
                );
            }

            "join" => {
                let input_token1_record_ciphertext = &record.inputs[0].value;
                let input_token2_record_ciphertext = &record.inputs[1].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
            }

            "split" => {
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token1_record_ciphertext = &record.outputs[0].value;
                let output_token2_record_ciphertext = &record.outputs[1].value;
            }

            "fee" => {
                let owner = &record.finalize[0];
                let token_info_id = &record.finalize[2];
                let hash_owner = bhp_256_hash(&owner).unwrap();
                let hash_id = bhp_256_hash(&token_info_id).unwrap();

                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = &hash_owner.add(hash_id).to_string();

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    &token_infos_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_token_info(
                    conn,
                    models::TokenInfos {
                        id: todo!(),
                        name: todo!(),
                        symbol: todo!(),
                        supply: todo!(),
                        decimals: todo!(),
                        max_mint_amount: todo!(),
                        minted_amount: todo!(),
                        dao_id: todo!(),
                        only_creator_can_mint: todo!(),
                    },
                );

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_BALANCES.to_string(),
                    balances_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_balances(
                    conn,
                    models::Balances {
                        key: balances_mapping_key.clone(),
                        owner: hold_token.token_owner,
                        amount: hold_token.amount as i64,
                        token_info_id: hold_token.token_info_id as i64,
                    },
                );
            }

            "update_profile" => {
                let profiles_mapping_key = &record.finalize[0];

                let profile: Profile = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_PROFILES.to_string(),
                    profiles_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_profile(
                    conn,
                    models::Profiles {
                        address: profiles_mapping_key.clone(),
                        name: profile.name,
                        avatar: profile.avatar,
                        bio: profile.bio,
                    },
                );
            }

            "update_time" => {
                let timestamp: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_TIMESTAMP.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: AUTO_INCREMENT_TIMESTAMP as i64,
                        value: timestamp as i64,
                    },
                );
            }

            "create_dao" => {
                let daos_mapping_key: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_DAOS.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let token_infos_mapping_key: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_TOKEN_INFOS.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    &token_infos_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                insert_token_info(
                    conn,
                    models::TokenInfos {
                        id: token_info.id as i64,
                        name: token_info.name,
                        symbol: token_info.symbol,
                        supply: token_info.supply as i64,
                        decimals: token_info.decimals as i64,
                        max_mint_amount: token_info.max_mint_amount as i64,
                        minted_amount: token_info.minted_amount as i64,
                        dao_id: token_info.dao_id as i64,
                        only_creator_can_mint: token_info.only_creator_can_mint,
                    },
                );

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_DAOS.to_string(),
                    &daos_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                create_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creater: dao.creator,
                        token_info_id: dao.token_info_id as i64,
                        icon: dao.icon,
                        description: dao.description,
                        official_link: dao.official_link,
                        proposal_count: dao.proposal_count as i64,
                        pass_proposal_count: dao.pass_proposal_count as i64,
                        vote_count: dao.vote_count as i64,
                        passed_votes_proportion: dao.passed_votes_proportion as i64,
                        passed_tokens_proportion: dao.passed_tokens_proportion as i64,
                    },
                );
            }

            "update_dao" => {
                // TODO: Processing Data
                let daos_mapping_key = &record.finalize[1];

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_DAOS.to_string(),
                    daos_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                create_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creater: dao.creator,
                        token_info_id: dao.token_info_id as i64,
                        icon: dao.icon,
                        description: dao.description,
                        official_link: dao.official_link,
                        proposal_count: dao.proposal_count as i64,
                        pass_proposal_count: dao.pass_proposal_count as i64,
                        vote_count: dao.vote_count as i64,
                        passed_votes_proportion: dao.passed_votes_proportion as i64,
                        passed_tokens_proportion: dao.passed_tokens_proportion as i64,
                    },
                );
            }

            "create_proposal" => {
                let proposals_mapping_key: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_PROPOSALS.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    &proposals_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                create_proposal(
                    conn,
                    models::Proposals {
                        id: proposal.id as i64,
                        title: proposal.title,
                        proposer: proposal.proposer,
                        summary: proposal.summary,
                        body: proposal.body,
                        dao_id: proposal.dao_id as i64,
                        created: proposal.created as i64,
                        duration: proposal.duration as i64,
                        proposer_type: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );
            }

            "start_proposal" => {
                let input_token_record_ciphertext = &record.inputs[2].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let proposals_mapping_key = &record.finalize[1];

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    proposals_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_proposal(
                    conn,
                    models::Proposals {
                        id: proposal.id as i64,
                        title: proposal.title,
                        proposer: proposal.proposer,
                        summary: proposal.summary,
                        body: proposal.body,
                        dao_id: proposal.dao_id as i64,
                        created: proposal.created as i64,
                        duration: proposal.duration as i64,
                        proposer_type: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );
            }

            "close_proposal" => {
                // TODO: Processing Data
                let proposals_mapping_key = &record.finalize[1];
                let daos_mapping_key: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_DAOS.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                let extend_pledge_period_mapping_key = &record.finalize[1];

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    &proposals_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_proposal(
                    conn,
                    models::Proposals {
                        id: proposal.id as i64,
                        title: proposal.title,
                        proposer: proposal.proposer,
                        summary: proposal.summary,
                        body: proposal.body,
                        dao_id: proposal.dao_id as i64,
                        created: proposal.created as i64,
                        duration: proposal.duration as i64,
                        proposer_type: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_DAOS.to_string(),
                    &daos_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creater: dao.creator,
                        token_info_id: dao.token_info_id as i64,
                        icon: dao.icon,
                        description: dao.description,
                        official_link: dao.official_link,
                        proposal_count: dao.proposal_count as i64,
                        pass_proposal_count: dao.pass_proposal_count as i64,
                        vote_count: dao.vote_count as i64,
                        passed_votes_proportion: dao.passed_votes_proportion as i64,
                        passed_tokens_proportion: dao.passed_tokens_proportion as i64,
                    },
                );

                let extend_pledge_period: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_EXTEND_PLEDGE_PERIOD.to_string(),
                    &extend_pledge_period_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                create_extend_pledge_period(
                    conn,
                    models::ExtendPledgePeriod {
                        key: extend_pledge_period_mapping_key.parse::<i64>().unwrap(),
                        value: extend_pledge_period as i64,
                    },
                );
            }

            "vote" => {
                let input_token_record_ciphertext = &record.inputs[1].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let proposals_mapping_key = &record.finalize[0];

                let daos_mapping_key: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_DAOS.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let votes_mapping_key: u64 = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_VOTES.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    &proposals_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_proposal(
                    conn,
                    models::Proposals {
                        id: proposal.id as i64,
                        title: proposal.title,
                        proposer: proposal.proposer,
                        summary: proposal.summary,
                        body: proposal.body,
                        dao_id: proposal.dao_id as i64,
                        created: proposal.created as i64,
                        duration: proposal.duration as i64,
                        proposer_type: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_DAOS.to_string(),
                    &daos_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                update_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creater: dao.creator,
                        token_info_id: dao.token_info_id as i64,
                        icon: dao.icon,
                        description: dao.description,
                        official_link: dao.official_link,
                        proposal_count: dao.proposal_count as i64,
                        pass_proposal_count: dao.pass_proposal_count as i64,
                        vote_count: dao.vote_count as i64,
                        passed_votes_proportion: dao.passed_votes_proportion as i64,
                        passed_tokens_proportion: dao.passed_tokens_proportion as i64,
                    },
                );

                let vote: Vote = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_VOTES.to_string(),
                    &votes_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                insert_votes(
                    conn,
                    models::Votes {
                        key: votes_mapping_key.to_string(),
                        voter: vote.voter,
                        proposal_id: vote.proposal_id as i64,
                        is_agreed: vote.is_agreed,
                        time: vote.timestamp as i64,
                        amount: vote.amount as i64,
                    },
                );
            }

            "init" => {
                let daos_mapping_key = 0u64;
                let token_infos_mapping_key = 0u64;

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_DAOS.to_string(),
                    &daos_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                create_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creater: dao.creator,
                        token_info_id: dao.token_info_id as i64,
                        icon: dao.icon,
                        description: dao.description,
                        official_link: dao.official_link,
                        proposal_count: dao.proposal_count as i64,
                        pass_proposal_count: dao.pass_proposal_count as i64,
                        vote_count: dao.vote_count as i64,
                        passed_votes_proportion: dao.passed_votes_proportion as i64,
                        passed_tokens_proportion: dao.passed_tokens_proportion as i64,
                    },
                );

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    &token_infos_mapping_key.to_string(),
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                insert_token_info(
                    conn,
                    models::TokenInfos {
                        id: token_info.id as i64,
                        name: token_info.name,
                        symbol: token_info.symbol,
                        supply: token_info.supply as i64,
                        decimals: token_info.decimals as i64,
                        max_mint_amount: token_info.max_mint_amount as i64,
                        minted_amount: token_info.minted_amount as i64,
                        dao_id: token_info.dao_id as i64,
                        only_creator_can_mint: token_info.only_creator_can_mint,
                    },
                );
            }

            _ => {}
        }
    }
}
