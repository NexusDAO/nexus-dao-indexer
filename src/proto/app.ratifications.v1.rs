#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ratifications {
    #[prost(message, repeated, tag="1")]
    pub ratifications: ::prost::alloc::vec::Vec<Ratify>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ratify {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub height: u32,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="4")]
    pub starting_round: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub total_stake: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub block_reward: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub puzzle_reward: ::core::option::Option<u64>,
}
