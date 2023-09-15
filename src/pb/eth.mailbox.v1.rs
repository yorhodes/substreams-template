// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dispatches {
    #[prost(message, repeated, tag="1")]
    pub dispatches: ::prost::alloc::vec::Vec<Dispatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dispatch {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub destination: u32,
    #[prost(string, tag="3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub body: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
    #[prost(string, tag="6")]
    pub trx_hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
