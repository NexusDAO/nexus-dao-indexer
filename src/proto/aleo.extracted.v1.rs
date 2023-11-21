#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extracted {
    #[prost(message, repeated, tag="1")]
    pub ratifications: ::prost::alloc::vec::Vec<Ratify>,
    #[prost(message, repeated, tag="2")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(enumeration="operation::OperationType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub program_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mapping_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub key_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub value_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub mapping_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="7")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        InitializeMapping = 0,
        InsertKeyValue = 1,
        UpdateKeyValue = 2,
        RemoveKeyValue = 3,
        RemoveMapping = 4,
    }
}
