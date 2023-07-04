use crate::{
    models::{Balances, Daos, ExtendPledgePeriod, Proposals, StakeAmounts, TokenInfos, Votes},
    proto::Records,
};
use anyhow::Error;
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

pub fn program_handler(rest_api: &String, records: &Records) {
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

                // TODO: Processing Data
                let output_token_record_ciphertext = &record.outputs[0].value;
                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = &hash_owner.add(hash_id).to_string();

                let token_infos: TokenInfos = match fetch_mapping(
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

                let balances: Balances = match fetch_mapping(
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
            }

            "stake" => {
                let hash_owner: Field<CurrentNetwork> =
                    Field::from_str(&record.finalize[0]).unwrap();
                let token_info_id = &record.finalize[2];
                let hash_id = bhp_256_hash(token_info_id).unwrap();

                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token1_record_ciphertext = &record.outputs[0].value;
                let output_token2_record_ciphertext = &record.outputs[1].value;
                let stake_amounts_mapping_key = &hash_owner.add(hash_id).to_string();

                let stake_amounts: StakeAmounts = match fetch_mapping(
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
            }

            "unstake" => {
                let hash_owner: Field<CurrentNetwork> =
                    Field::from_str(&record.finalize[1]).unwrap();
                let token_info_id = &record.finalize[3];
                let hash_id = bhp_256_hash(&token_info_id).unwrap();

                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let stake_amounts_mapping_key = &hash_owner.add(hash_id).to_string();

                let stake_amounts: StakeAmounts = match fetch_mapping(
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
            }

            "transfer" => {
                let sender = &record.finalize[0];
                let receiver = &record.finalize[1];
                let token_info_id = &record.finalize[3];

                let hash_id = bhp_256_hash(&token_info_id).unwrap();
                let sender_hash = bhp_256_hash(&sender).unwrap();
                let receiver_hash = bhp_256_hash(&receiver).unwrap();

                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token1_record_ciphertext = &record.outputs[0].value;
                let output_token2_record_ciphertext = &record.outputs[1].value;
                let balances1_mapping_key = &sender_hash.add(hash_id).to_string();
                let balances2_mapping_key = &receiver_hash.add(hash_id).to_string();

                let balances1: Balances = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_BALANCES.to_string(),
                    balances1_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };

                let balances2: Balances = match fetch_mapping(
                    rest_api,
                    &PROGRAM_ID.to_string(),
                    &MAPPING_NAME_BALANCES.to_string(),
                    balances2_mapping_key,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        println!("Fetch mapping error {:#}", err);
                        continue;
                    }
                };
            }

            "join" => {
                // TODO: Processing Data
                let input_token1_record_ciphertext = &record.inputs[0].value;
                let input_token2_record_ciphertext = &record.inputs[1].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
            }

            "split" => {
                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token1_record_ciphertext = &record.outputs[0].value;
                let output_token2_record_ciphertext = &record.outputs[1].value;
            }

            "fee" => {
                let owner = &record.finalize[0];
                let token_info_id = &record.finalize[2];
                let hash_owner = bhp_256_hash(&owner).unwrap();
                let hash_id = bhp_256_hash(&token_info_id).unwrap();

                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = &hash_owner.add(hash_id).to_string();

                let token_infos: TokenInfos = match fetch_mapping(
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

                let balances: Balances = match fetch_mapping(
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
            }

            "update_profile" => {
                // TODO: Processing Data
                let profiles_mapping_key = &record.finalize[0];

                let profiles: Balances = match fetch_mapping(
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
            }

            "update_time" => {
                // TODO: Processing Data
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
            }

            "create_dao" => {
                // TODO: Processing Data
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

                let daos: Daos = match fetch_mapping(
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
            }

            "update_dao" => {
                // TODO: Processing Data
                let daos_mapping_key = &record.finalize[1];

                let daos: Daos = match fetch_mapping(
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
            }

            "create_proposal" => {
                // TODO: Processing Data
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

                let proposals: Proposals = match fetch_mapping(
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
            }

            "start_proposal" => {
                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[2].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let proposals_mapping_key = &record.finalize[1];

                let proposals: Proposals = match fetch_mapping(
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

                let proposals: Proposals = match fetch_mapping(
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

                let daos: Daos = match fetch_mapping(
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

                let extend_pledge_period: ExtendPledgePeriod = match fetch_mapping(
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
            }

            "vote" => {
                // TODO: Processing Data
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

                let proposals: Proposals = match fetch_mapping(
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

                let daos: Daos = match fetch_mapping(
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

                let votes: Votes = match fetch_mapping(
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
            }

            "init" => {
                // TODO: Processing Data
                let daos_mapping_key = 0u64;
                let token_infos_mapping_key = 0u64;

                let daos: Daos = match fetch_mapping(
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

                let token_infos: TokenInfos = match fetch_mapping(
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
            }

            _ => {}
        }
    }
}
