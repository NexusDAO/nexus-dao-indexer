use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub proposer: String,
    pub summary: String,
    pub body: String,
    pub dao_id: u64,
    pub created: u64,
    pub duration: u64,
    pub proposal_type: u32,
    pub adopt: u64,
    pub reject: u64,
    pub status: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub avatar: String,
    pub bio: String,
}

#[derive(Deserialize, Serialize)]
pub struct DaoSchema {
    pub name: String,
    pub dao_type: u32,
    pub creater: String,
    pub icon: String,
    pub description: String,
    pub official_link: String,
}

#[derive(Deserialize, Serialize)]
pub struct Dao {
    pub id: u64,
    pub name: String,
    pub dao_type: u32,
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

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub owner: String,
    pub gates: u64,
    pub token_info_id: u64,
    pub amount: u64,
    pub expires: u64,
    pub staked_at: u64,
}

#[derive(Deserialize, Serialize)]
pub struct HoldToken {
    pub token_owner: String,
    pub amount: u64,
    pub token_info_id: u64,
}

#[derive(Deserialize, Serialize)]
pub struct TokenInfo {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub decimals: u32,
    pub max_mint_amount: u64,
    pub minted_amount: u64,
    pub dao_id: u64,
    pub only_creator_can_mint: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TokenInfoSchema {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub decimals: u32,
    pub max_mint_amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Vote {
    pub voter: String,
    pub proposal_id: u64,
    pub is_agreed: bool,
    pub timestamp: u64,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
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
