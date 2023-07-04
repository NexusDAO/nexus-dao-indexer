use crate::{
    models,
    proto::{Proposal, Records},
    schema::record::function,
};
use anyhow::{Error, Ok};
use regex::Regex;
use snarkvm::{prelude::*, utilities::ToBits};

type CurrentNetwork = snarkvm::prelude::Testnet3;

const AUTO_INCREMENT_TIMESTAMP: &str = "0";
const AUTO_INCREMENT_TOKEN_INFOS: &str = "1";
const AUTO_INCREMENT_PROPOSALS: &str = "2";
const AUTO_INCREMENT_DAOS: &str = "3";
const AUTO_INCREMENT_VOTES: &str = "4";
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
    ureq::get(&format!(
        "{rest_api}/testnet3/program/{program_id}/mapping/{mapping_name}/{mapping_key}"
    ))
    .call()?
    .into_json()
    .map_err(|e| e.into())
}

pub fn program_handler(rest_api: &String, records: &Records) {
    let program_id = "pledge_vote.aleo";

    for record in records.records.iter() {
        if record.program != program_id {
            continue;
        };

        // let json_str = Regex::new(r#"(\w+)"#)
        //     .unwrap()
        //     .replace_all(&record.inputs.first().unwrap().value, r#""$1""#)
        //     .to_string();

        match record.function.as_str() {
            "mint" => {
                let owner = &record.finalize[0];
                let hash_owner = bhp_256_hash(owner).unwrap();
                let token_info_id = &record.finalize[2];
                let hash_id = bhp_256_hash(&token_info_id).unwrap();

                // TODO: Processing Data
                let output_token_record_ciphertext = &record.outputs[0].value;
                let token_infos_mapping_key = token_info_id;
                let balances_mapping_key = hash_owner.add(hash_id).to_string();
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
                let stake_amounts_mapping_key = hash_owner.add(hash_id).to_string();
            }

            "unstake" => {
                let hash_owner: Field<CurrentNetwork> =
                    Field::from_str(&record.finalize[1]).unwrap();
                let token_info_id = &record.finalize[3];
                let hash_id = bhp_256_hash(&token_info_id).unwrap();

                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[0].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let stake_amounts_mapping_key = hash_owner.add(hash_id).to_string();
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
                let balances1_mapping_key = sender_hash.add(hash_id).to_string();
                let balances2_mapping_key = receiver_hash.add(hash_id).to_string();
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
                let balances_mapping_key = hash_owner.add(hash_id).to_string();
            }

            "update_profile" => {
                // TODO: Processing Data
                let profiles_mapping_key = &record.finalize[0];
            }

            "update_time" => {
                // TODO: Processing Data
                let auto_increment_mapping_key = 0u8;
            }

            "create_dao" => {
                // TODO: Processing Data
                let daos_mapping_key: u64 = fetch_mapping(
                    rest_api,
                    &program_id.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_DAOS.to_string(),
                )
                .unwrap();

                let token_infos_mapping_key: u64 = fetch_mapping(
                    rest_api,
                    &program_id.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_TOKEN_INFOS.to_string(),
                )
                .unwrap();
            }

            "update_dao" => {
                // TODO: Processing Data
                let daos_mapping_key = &record.finalize[1];
            }

            "create_proposal" => {
                // TODO: Processing Data
                let auto_increment_mapping_key = 2u8;
                let proposals_mapping_key: u64 = fetch_mapping(
                    rest_api,
                    &program_id.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_PROPOSALS.to_string(),
                )
                .unwrap();
            }

            "start_proposal" => {
                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[2].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let proposals_mapping_key = &record.finalize[1];
            }

            "close_proposal" => {
                // TODO: Processing Data
                let proposals_mapping_key = &record.finalize[1];
                let daos_mapping_key: u64 = fetch_mapping(
                    rest_api,
                    &program_id.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_DAOS.to_string(),
                )
                .unwrap();
                let extend_pledge_period_mapping_key = &record.finalize[1];
            }

            "vote" => {
                // TODO: Processing Data
                let input_token_record_ciphertext = &record.inputs[1].value;
                let output_token_record_ciphertext = &record.outputs[0].value;
                let proposals_mapping_key = &record.finalize[0];
                let daos_mapping_key: u64 = fetch_mapping(
                    rest_api,
                    &program_id.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_DAOS.to_string(),
                )
                .unwrap();
                let auto_increment_mapping_key = 4u8;
                let votes_mapping_key: u64 = fetch_mapping(
                    rest_api,
                    &program_id.to_string(),
                    &MAPPING_NAME_AUTO_INCREMENT.to_string(),
                    &AUTO_INCREMENT_VOTES.to_string(),
                )
                .unwrap();
            }

            "init" => {
                // TODO: Processing Data
                let auto_increment0_mapping_key = 0u8;
                let auto_increment1_mapping_key = 1u8;
                let auto_increment2_mapping_key = 2u8;
                let auto_increment3_mapping_key = 3u8;
                let auto_increment4_mapping_key = 4u8;
                let daos_mapping_key = 0u64;
                let token_infos_mapping_key = 0u64;
            }

            _ => {}
        }

        // let mapping = serde_json::from_str(&json_str).unwrap();
        // match mapping {
        //   Proposal{ id, title, proposer, summary, body, dao_id, created, duration, proposal_type, status } => {
        //     println!("FIRE PROPOSAL {:?}", id);
        //   }
        //   Profile{ name, avatar, bio } => {
        //     println!("FIRE PROPOSAL {:?}", name);
        //   }
        // }
    }
}
