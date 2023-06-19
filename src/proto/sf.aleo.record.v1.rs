#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Records {
    #[prost(message, repeated, tag="1")]
    pub records: ::prost::alloc::vec::Vec<Record>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    #[prost(string, tag="1")]
    pub program: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub function: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    #[prost(message, repeated, tag="4")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
    #[prost(string, tag="5")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub previous_hash: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub transition_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub network: u32,
    #[prost(uint32, tag="10")]
    pub height: u32,
    #[prost(int64, tag="11")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub checksum: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
}
