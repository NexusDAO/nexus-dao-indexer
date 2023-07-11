use anyhow::Error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn revert_to_str_map(value: &String) -> Result<HashMap<String, String>, Error> {
    let json_str = Regex::new(r#"(\w+)"#)
        .unwrap()
        .replace_all(value.replace(r"\n", "").trim_matches('"'), r#""$1""#)
        .to_string();
    let mapping = serde_json::from_str(&json_str)?;
    Ok(mapping)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub proposer: String,
    pub summary: String,
    pub body: String,
    pub dao_id: u64,
    pub created: u32,
    pub duration: u32,
    pub proposal_type: u8,
    pub adopt: u64,
    pub reject: u64,
    pub status: u8,
}

impl Proposal {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            id: data["id"].trim_end_matches("u64").parse::<u64>()?,
            title: data["title"].trim_end_matches("field").to_string(),
            proposer: data["proposer"].clone(),
            summary: data["summary"].trim_end_matches("field").to_string(),
            body: data["body"].trim_end_matches("field").to_string(),
            dao_id: data["dao_id"].trim_end_matches("u64").parse::<u64>()?,
            created: data["created"].trim_end_matches("u32").parse::<u32>()?,
            duration: data["duration"].trim_end_matches("u32").parse::<u32>()?,
            proposal_type: data["proposal_type"].trim_end_matches("u8").parse::<u8>()?,
            adopt: data["adopt"].trim_end_matches("u64").parse::<u64>()?,
            reject: data["reject"].trim_end_matches("u64").parse::<u64>()?,
            status: data["status"].trim_end_matches("u8").parse::<u8>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Profile {
    pub name: String,
    pub avatar: String,
    pub bio: String,
}

impl Profile {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            name: data["name"].trim_end_matches("field").to_string(),
            avatar: data["avatar"].trim_end_matches("field").to_string(),
            bio: data["bio"].trim_end_matches("field").to_string(),
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dao {
    pub id: u64,
    pub name: String,
    pub dao_type: u8,
    pub creator: String,
    pub token_info_id: u64,
    pub icon: String,
    pub description: String,
    pub official_link: String,
    pub proposal_count: u64,
    pub pass_proposal_count: u64,
    pub vote_count: u64,
    pub passed_votes_proportion: u64,
    pub passed_tokens_proportion: u64,
}

impl Dao {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            id: data["id"].trim_end_matches("u64").parse::<u64>()?,
            name: data["name"].trim_end_matches("field").to_string(),
            dao_type: data["dao_type"].trim_end_matches("u8").parse::<u8>()?,
            creator: data["creator"].to_string(),
            token_info_id: data["token_info_id"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            icon: data["icon"].trim_end_matches("field").to_string(),
            description: data["description"].trim_end_matches("field").to_string(),
            official_link: data["official_link"].trim_end_matches("field").to_string(),
            proposal_count: data["proposal_count"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            pass_proposal_count: data["pass_proposal_count"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            vote_count: data["vote_count"].trim_end_matches("u64").parse::<u64>()?,
            passed_votes_proportion: data["passed_votes_proportion"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            passed_tokens_proportion: data["passed_tokens_proportion"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub owner: String,
    pub gates: u64,
    pub token_info_id: u64,
    pub amount: u64,
    pub expires: u32,
    pub staked_at: u64,
}

impl Token {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            owner: data["owner"].to_string(),
            gates: data["gates"].trim_end_matches("u64").parse::<u64>()?,
            token_info_id: data["token_info_id"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            amount: data["amount"].trim_end_matches("u64").parse::<u64>()?,
            expires: data["expires"].trim_end_matches("u32").parse::<u32>()?,
            staked_at: data["staked_at"].trim_end_matches("u64").parse::<u64>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HoldToken {
    pub token_owner: String,
    pub amount: u64,
    pub token_info_id: u64,
}

impl HoldToken {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            token_owner: data["token_owner"].to_string(),
            amount: data["amount"].trim_end_matches("u64").parse::<u64>()?,
            token_info_id: data["token_info_id"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenInfo {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub decimals: u8,
    pub max_mint_amount: u64,
    pub minted_amount: u64,
    pub dao_id: u64,
    pub only_creator_can_mint: bool,
}

impl TokenInfo {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            id: data["id"].trim_end_matches("u64").parse::<u64>()?,
            name: data["name"].trim_end_matches("field").to_string(),
            symbol: data["symbol"].trim_end_matches("field").to_string(),
            supply: data["supply"].trim_end_matches("u64").parse::<u64>()?,
            decimals: data["decimals"].trim_end_matches("u8").parse::<u8>()?,
            max_mint_amount: data["max_mint_amount"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            minted_amount: data["minted_amount"]
                .trim_end_matches("u64")
                .parse::<u64>()?,
            dao_id: data["dao_id"].trim_end_matches("u64").parse::<u64>()?,
            only_creator_can_mint: data["only_creator_can_mint"].parse::<bool>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vote {
    pub voter: String,
    pub proposal_id: u64,
    pub is_agreed: bool,
    pub time: u32,
    pub amount: u64,
}

impl Vote {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = revert_to_str_map(value)?;
        Ok(Self {
            voter: data["voter"].trim_end_matches("field").to_string(),
            proposal_id: data["proposal_id"].trim_end_matches("u64").parse::<u64>()?,
            is_agreed: data["is_agreed"].parse::<bool>()?,
            time: data["time"].trim_end_matches("u32").parse::<u32>()?,
            amount: data["amount"].trim_end_matches("u64").parse::<u64>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AutoIncrement {
    pub value: u64,
}

impl AutoIncrement {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = value.trim_matches('"');
        Ok(Self {
            value: data.trim_end_matches("u64").parse::<u64>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtendPledgePeriod {
    pub value: u64,
}

impl ExtendPledgePeriod {
    pub fn from_mapping_value(value: &String) -> Result<Self, Error> {
        let data = value.trim_matches('"');
        Ok(Self {
            value: data.trim_end_matches("u64").parse::<u64>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MappingAutoIncrement {
    pub auto_increment: HashMap<u32, u64>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingProfiles {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingDaos {
    pub daos: HashMap<u64, Dao>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingTokenInfos {
    pub token_infos: HashMap<u64, TokenInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingBalances {
    pub balances: HashMap<String, HoldToken>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingStakeAmounts {
    pub stake_amounts: HashMap<String, HoldToken>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingProposals {
    pub proposals: HashMap<u64, Proposal>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingVotes {
    pub votes: HashMap<u64, Vote>,
}

#[derive(Deserialize, Serialize)]
pub struct MappingExtendPledgePeriod {
    pub extend_pledge_period: HashMap<u64, u64>,
}
