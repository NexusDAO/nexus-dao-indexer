#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub proposer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub body: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub dao_id: u64,
    #[prost(uint64, tag="7")]
    pub created: u64,
    #[prost(uint64, tag="8")]
    pub duration: u64,
    #[prost(uint32, tag="9")]
    pub proposal_type: u32,
    #[prost(uint64, tag="10")]
    pub adopt: u64,
    #[prost(uint64, tag="11")]
    pub reject: u64,
    #[prost(uint32, tag="12")]
    pub status: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub avatar: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bio: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaoSchema {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub dao_type: u32,
    #[prost(string, tag="3")]
    pub creater: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub icon: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub official_link: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dao {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub dao_type: u32,
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub token_info_id: u64,
    #[prost(string, tag="6")]
    pub icon: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub official_link: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub proposal_count: u64,
    #[prost(uint64, tag="10")]
    pub pass_proposal_count: u64,
    #[prost(uint64, tag="11")]
    pub vote_count: u64,
    #[prost(uint64, tag="12")]
    pub passed_votes_proportion: u64,
    #[prost(uint64, tag="13")]
    pub passed_tokens_proportion: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub gates: u64,
    #[prost(uint64, tag="3")]
    pub token_info_id: u64,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(uint64, tag="5")]
    pub expires: u64,
    #[prost(uint64, tag="6")]
    pub staked_at: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldToken {
    #[prost(string, tag="1")]
    pub token_owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(uint64, tag="3")]
    pub token_info_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenInfo {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub supply: u64,
    #[prost(uint32, tag="5")]
    pub decimals: u32,
    #[prost(uint64, tag="6")]
    pub max_mint_amount: u64,
    #[prost(uint64, tag="7")]
    pub minted_amount: u64,
    #[prost(uint64, tag="8")]
    pub dao_id: u64,
    #[prost(bool, tag="9")]
    pub only_creator_can_mint: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenInfoSchema {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub supply: u64,
    #[prost(uint32, tag="4")]
    pub decimals: u32,
    #[prost(uint64, tag="5")]
    pub max_mint_amount: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(string, tag="1")]
    pub voter: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub proposal_id: u64,
    #[prost(bool, tag="3")]
    pub is_agreed: bool,
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
    #[prost(uint64, tag="5")]
    pub amount: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingAutoIncrement {
    #[prost(map="uint32, uint64", tag="1")]
    pub auto_increment: ::std::collections::HashMap<u32, u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingProfiles {
    #[prost(map="string, message", tag="1")]
    pub profiles: ::std::collections::HashMap<::prost::alloc::string::String, Profile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingDaos {
    #[prost(map="uint64, message", tag="1")]
    pub daos: ::std::collections::HashMap<u64, Dao>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingTokenInfos {
    #[prost(map="uint64, message", tag="1")]
    pub token_infos: ::std::collections::HashMap<u64, TokenInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingBalances {
    #[prost(map="string, message", tag="1")]
    pub balances: ::std::collections::HashMap<::prost::alloc::string::String, HoldToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingStakeAmounts {
    #[prost(map="string, message", tag="1")]
    pub stake_amounts: ::std::collections::HashMap<::prost::alloc::string::String, HoldToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingProposals {
    #[prost(map="uint64, message", tag="1")]
    pub proposals: ::std::collections::HashMap<u64, Proposal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingVotes {
    #[prost(map="uint64, message", tag="1")]
    pub votes: ::std::collections::HashMap<u64, Vote>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingExtendPledgePeriod {
    #[prost(map="uint64, uint64", tag="1")]
    pub extend_pledge_period: ::std::collections::HashMap<u64, u64>,
}
