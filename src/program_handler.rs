use crate::{
    database::{
        create_dao, create_extend_pledge_period, create_proposal, get_auto_increment_by_key,
        insert_votes, update_dao, update_proposal, update_token_info, upsert_auto_increment,
        upsert_balances, upsert_profile, upsert_stake_amounts, upsert_token_info,
    },
    mappings::{
        AutoIncrement, Dao, ExtendPledgePeriod, HoldToken, Profile, Proposal, TokenInfo, Vote,
    },
    models,
    proto::Records,
};
use anyhow::Error;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use snarkvm::console::program::Plaintext;
use snarkvm::prelude::{traits::ToBits, *};

type CurrentNetwork = Testnet3;

const INIT_VALUE_AUTO_INCREMENT_TOKEN_INFOS: i64 = 1;
const INIT_VALUE_AUTO_INCREMENT_PROPOSALS: i64 = 1;
const INIT_VALUE_AUTO_INCREMENT_DAOS: i64 = 1;
const INIT_VALUE_AUTO_INCREMENT_VOTES: i64 = 1;
const KEY_AUTO_INCREMENT_TIMESTAMP: i64 = 0;
const KEY_AUTO_INCREMENT_TOKEN_INFOS: i64 = 1;
const KEY_AUTO_INCREMENT_PROPOSALS: i64 = 2;
const KEY_AUTO_INCREMENT_DAOS: i64 = 3;
const KEY_AUTO_INCREMENT_VOTES: i64 = 4;
const MAPPING_KEY_AUTO_INCREMENT_TIMESTAMP: &str = "0u8";
const MAPPING_KEY_AUTO_INCREMENT_TOKEN_INFOS: &str = "1u8";
const MAPPING_KEY_AUTO_INCREMENT_PROPOSALS: &str = "2u8";
const MAPPING_KEY_AUTO_INCREMENT_DAOS: &str = "3u8";
const MAPPING_KEY_AUTO_INCREMENT_VOTES: &str = "4u8";
const MAPPING_NAME_AUTO_INCREMENT: &str = "auto_increment";
const MAPPING_NAME_PROFILES: &str = "profiles";
const MAPPING_NAME_DAOS: &str = "daos";
const MAPPING_NAME_TOKEN_INFOS: &str = "token_infos";
const MAPPING_NAME_BALANCES: &str = "balances";
const MAPPING_NAME_STAKE_AMOUNTS: &str = "stake_amounts";
const MAPPING_NAME_PROPOSALS: &str = "proposals";
const MAPPING_NAME_VOTES: &str = "votes";
const MAPPING_NAME_EXTEND_PLEDGE_PERIOD: &str = "extend_pledge_period";

pub fn bhp256_hash_address(addr: &String) -> Result<Field<Testnet3>, Error> {
    let field = Testnet3::hash_bhp256(
        &Plaintext::from(Literal::Address(
            Address::<Testnet3>::parse(addr).unwrap().1,
        ))
        .to_bits_le(),
    )?;
    Ok(field)
}

pub fn bhp256_hash_u64(value: u64) -> Result<Field<Testnet3>, Error> {
    let plaintext: Plaintext<Testnet3> = Plaintext::from(Literal::U64(U64::new(value)));
    let field = Testnet3::hash_bhp256(&plaintext.to_bits_le())?;
    Ok(field)
}

fn fetch_mapping(
    rest_api: &String,
    program_id: &String,
    mapping_name: &String,
    mapping_key: &String,
) -> Result<String, Error> {
    let url =
        format!("{rest_api}/testnet3/program/{program_id}/mapping/{mapping_name}/{mapping_key}");
    let value = ureq::get(&url).call()?.into_string()?;

    if value == "null" {
        return Err(anyhow!("Mapping value is null (url {})", url));
    }

    Ok(value)
}

pub fn program_handler(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    rest_api: &String,
    records: &Records,
    program_id: &String,
) {
    for record in records.records.iter() {
        if record.program != *program_id {
            continue;
        };

        match record.function.as_str() {
            "mint" => {
                let owner = &record.finalize[0];
                let token_info_id = &record.finalize[2];

                let hash_owner = bhp256_hash_address(owner).unwrap();
                let hash_id = bhp256_hash_u64(
                    token_info_id
                        .trim_end_matches("u64")
                        .parse::<u64>()
                        .unwrap(),
                )
                .unwrap();

                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = &hash_owner.add(hash_id).to_string();

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    token_infos_mapping_key,
                ) {
                    Ok(data) => TokenInfo::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_BALANCES.to_string(),
                    balances_mapping_key,
                ) {
                    Ok(data) => HoldToken::from_mapping_value(&data).unwrap(),
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

                upsert_balances(
                    conn,
                    models::Balances {
                        key: balances_mapping_key.to_string(),
                        owner: owner.to_string(),
                        amount: hold_token.amount as i64,
                        token_info_id: hold_token.token_info_id as i64,
                    },
                );
            }

            "stake" => {
                let hash_owner: Field<CurrentNetwork> =
                    Field::from_str(&record.finalize[0]).unwrap();
                let token_info_id = &record.finalize[2];
                let hash_id = bhp256_hash_u64(
                    token_info_id
                        .trim_end_matches("u64")
                        .parse::<u64>()
                        .unwrap(),
                )
                .unwrap();

                let stake_amounts_mapping_key = &hash_owner.add(hash_id).to_string();

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_STAKE_AMOUNTS.to_string(),
                    stake_amounts_mapping_key,
                ) {
                    Ok(data) => HoldToken::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                upsert_stake_amounts(
                    conn,
                    models::StakeAmounts {
                        key: stake_amounts_mapping_key.to_string(),
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
                let hash_id = bhp256_hash_u64(
                    token_info_id
                        .trim_end_matches("u64")
                        .parse::<u64>()
                        .unwrap(),
                )
                .unwrap();

                let stake_amounts_mapping_key = &hash_owner.add(hash_id).to_string();

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_STAKE_AMOUNTS.to_string(),
                    stake_amounts_mapping_key,
                ) {
                    Ok(data) => HoldToken::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                upsert_stake_amounts(
                    conn,
                    models::StakeAmounts {
                        key: stake_amounts_mapping_key.to_string(),
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

                let hash_id = bhp256_hash_u64(
                    token_info_id
                        .trim_end_matches("u64")
                        .parse::<u64>()
                        .unwrap(),
                )
                .unwrap();
                let sender_hash = bhp256_hash_address(sender).unwrap();
                let receiver_hash = bhp256_hash_address(receiver).unwrap();

                let sender_balances_mapping_key = &sender_hash.add(hash_id).to_string();
                let receiver_balances_mapping_key = &receiver_hash.add(hash_id).to_string();

                let sender_hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_BALANCES.to_string(),
                    sender_balances_mapping_key,
                ) {
                    Ok(data) => HoldToken::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                upsert_balances(
                    conn,
                    models::Balances {
                        key: sender_balances_mapping_key.to_string(),
                        owner: sender.to_string(),
                        amount: sender_hold_token.amount as i64,
                        token_info_id: sender_hold_token.token_info_id as i64,
                    },
                );

                let receiver_hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_BALANCES.to_string(),
                    receiver_balances_mapping_key,
                ) {
                    Ok(data) => HoldToken::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                upsert_balances(
                    conn,
                    models::Balances {
                        key: receiver_balances_mapping_key.clone(),
                        owner: receiver.to_string(),
                        amount: receiver_hold_token.amount as i64,
                        token_info_id: receiver_hold_token.token_info_id as i64,
                    },
                );
            }

            "join" => {}

            "split" => {}

            "fee" => {
                let owner = &record.finalize[0];
                let token_info_id = &record.finalize[2];
                let hash_owner = bhp256_hash_address(owner).unwrap();
                let hash_id = bhp256_hash_u64(
                    token_info_id
                        .trim_end_matches("u64")
                        .parse::<u64>()
                        .unwrap(),
                )
                .unwrap();

                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = &hash_owner.add(hash_id).to_string();

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    &token_infos_mapping_key,
                ) {
                    Ok(data) => TokenInfo::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let hold_token: HoldToken = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_BALANCES.to_string(),
                    balances_mapping_key,
                ) {
                    Ok(data) => HoldToken::from_mapping_value(&data).unwrap(),
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

                upsert_balances(
                    conn,
                    models::Balances {
                        key: balances_mapping_key.clone(),
                        owner: owner.clone(),
                        amount: hold_token.amount as i64,
                        token_info_id: hold_token.token_info_id as i64,
                    },
                );
            }

            "update_profile" => {
                let profiles_mapping_key = &record.finalize[0];

                let profile: Profile = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_PROFILES.to_string(),
                    profiles_mapping_key,
                ) {
                    Ok(data) => Profile::from_mapping_value(&data).unwrap(),
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
                let timestamp = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &MAPPING_KEY_AUTO_INCREMENT_TIMESTAMP.to_string(),
                ) {
                    Ok(data) => AutoIncrement::from_mapping_value(&data).unwrap().value,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_TIMESTAMP,
                        value: timestamp as i64,
                    },
                );
            }

            "create_dao" => {
                let daos_mapping_key = get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_DAOS)
                    .unwrap_or_else(|_| {
                        upsert_auto_increment(
                            conn,
                            models::AutoIncrement {
                                key: KEY_AUTO_INCREMENT_DAOS,
                                value: INIT_VALUE_AUTO_INCREMENT_DAOS,
                            },
                        )
                        .unwrap();
                        get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_DAOS).unwrap()
                    })
                    .value;

                let token_infos_mapping_key =
                    get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_TOKEN_INFOS)
                        .unwrap_or_else(|_| {
                            upsert_auto_increment(
                                conn,
                                models::AutoIncrement {
                                    key: KEY_AUTO_INCREMENT_TOKEN_INFOS,
                                    value: INIT_VALUE_AUTO_INCREMENT_TOKEN_INFOS,
                                },
                            )
                            .unwrap();
                            get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_TOKEN_INFOS).unwrap()
                        })
                        .value;

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    &format!("{}{}", token_infos_mapping_key, "u64"),
                ) {
                    Ok(data) => TokenInfo::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_DAOS.to_string(),
                    &format!("{}{}", daos_mapping_key, "u64"),
                ) {
                    Ok(data) => Dao::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                upsert_token_info(
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

                create_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creator: dao.creator,
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

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_TOKEN_INFOS,
                        value: token_infos_mapping_key.add(1),
                    },
                );

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_DAOS,
                        value: daos_mapping_key.add(1),
                    },
                );
            }

            "update_dao" => {
                let daos_mapping_key = &record.finalize[1];

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_DAOS.to_string(),
                    daos_mapping_key,
                ) {
                    Ok(data) => Dao::from_mapping_value(&data).unwrap(),
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
                        creator: dao.creator,
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
                let proposals_mapping_key =
                    get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_PROPOSALS)
                        .unwrap_or_else(|_| {
                            upsert_auto_increment(
                                conn,
                                models::AutoIncrement {
                                    key: KEY_AUTO_INCREMENT_PROPOSALS,
                                    value: INIT_VALUE_AUTO_INCREMENT_PROPOSALS,
                                },
                            )
                            .unwrap();
                            get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_PROPOSALS).unwrap()
                        })
                        .value;

                let proposal = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    &format!("{}{}", proposals_mapping_key, "u64"),
                ) {
                    Ok(data) => Proposal::from_mapping_value(&data).unwrap(),
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
                        type_: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_PROPOSALS,
                        value: proposals_mapping_key.add(1),
                    },
                );
            }

            "start_proposal" => {
                let proposals_mapping_key = &record.finalize[1];

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    proposals_mapping_key,
                ) {
                    Ok(data) => Proposal::from_mapping_value(&data).unwrap(),
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
                        type_: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );
            }

            "close_proposal" => {
                let proposals_mapping_key = &record.finalize[1];
                let daos_mapping_key = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &MAPPING_KEY_AUTO_INCREMENT_DAOS.to_string(),
                ) {
                    Ok(data) => AutoIncrement::from_mapping_value(&data).unwrap().value,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
                let extend_pledge_period_mapping_key = &record.finalize[1];

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    &proposals_mapping_key.to_string(),
                ) {
                    Ok(data) => Proposal::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_DAOS.to_string(),
                    &format!("{}{}", daos_mapping_key, "u64"),
                ) {
                    Ok(data) => Dao::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let extend_pledge_period: ExtendPledgePeriod = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_EXTEND_PLEDGE_PERIOD.to_string(),
                    &extend_pledge_period_mapping_key.to_string(),
                ) {
                    Ok(data) => ExtendPledgePeriod::from_mapping_value(&data).unwrap(),
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
                        type_: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );

                update_dao(
                    conn,
                    models::Daos {
                        id: dao.id as i64,
                        name: dao.name,
                        dao_type: dao.dao_type as i64,
                        creator: dao.creator,
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

                create_extend_pledge_period(
                    conn,
                    models::ExtendPledgePeriod {
                        key: extend_pledge_period_mapping_key.parse::<i64>().unwrap(),
                        value: extend_pledge_period.value as i64,
                    },
                );
            }

            "vote" => {
                let proposals_mapping_key = &record.finalize[0];

                let proposal: Proposal = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_PROPOSALS.to_string(),
                    &proposals_mapping_key.to_string(),
                ) {
                    Ok(data) => Proposal::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let daos_mapping_key = proposal.dao_id;
                let dao: Dao = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_DAOS.to_string(),
                    &format!("{}{}", daos_mapping_key, "u64"),
                ) {
                    Ok(data) => Dao::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let votes_mapping_key: i64 =
                    get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_VOTES)
                        .unwrap_or_else(|_| {
                            upsert_auto_increment(
                                conn,
                                models::AutoIncrement {
                                    key: KEY_AUTO_INCREMENT_VOTES,
                                    value: INIT_VALUE_AUTO_INCREMENT_VOTES,
                                },
                            )
                            .unwrap();
                            get_auto_increment_by_key(conn, KEY_AUTO_INCREMENT_VOTES).unwrap()
                        })
                        .value;

                let vote: Vote = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_VOTES.to_string(),
                    &format!("{}{}", votes_mapping_key, "u64"),
                ) {
                    Ok(data) => Vote::from_mapping_value(&data).unwrap(),
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
                        creator: dao.creator,
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
                        type_: proposal.proposal_type as i64,
                        adopt: proposal.adopt as i64,
                        reject: proposal.reject as i64,
                        status: proposal.status as i64,
                    },
                );

                insert_votes(
                    conn,
                    models::Votes {
                        key: votes_mapping_key.to_string(),
                        voter: vote.voter,
                        proposal_id: vote.proposal_id as i64,
                        is_agreed: vote.is_agreed,
                        time: vote.time as i64,
                        amount: vote.amount as i64,
                    },
                );

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_VOTES,
                        value: votes_mapping_key.add(1),
                    },
                );
            }

            "init" => {
                let token_infos_mapping_key = 0u64;
                let daos_mapping_key = 0u64;

                let dao: Dao = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_DAOS.to_string(),
                    &format!("{}{}", daos_mapping_key, "u64"),
                ) {
                    Ok(data) => Dao::from_mapping_value(&data).unwrap(),
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let token_info: TokenInfo = match fetch_mapping(
                    rest_api,
                    program_id,
                    &MAPPING_NAME_TOKEN_INFOS.to_string(),
                    &format!("{}{}", token_infos_mapping_key, "u64"),
                ) {
                    Ok(data) => TokenInfo::from_mapping_value(&data).unwrap(),
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
                        creator: dao.creator,
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

                upsert_token_info(
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

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_DAOS,
                        value: INIT_VALUE_AUTO_INCREMENT_DAOS,
                    },
                );

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_TOKEN_INFOS,
                        value: INIT_VALUE_AUTO_INCREMENT_TOKEN_INFOS,
                    },
                );

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_PROPOSALS,
                        value: INIT_VALUE_AUTO_INCREMENT_PROPOSALS,
                    },
                );

                upsert_auto_increment(
                    conn,
                    models::AutoIncrement {
                        key: KEY_AUTO_INCREMENT_VOTES,
                        value: INIT_VALUE_AUTO_INCREMENT_VOTES,
                    },
                );
            }

            _ => {}
        }
    }
}
