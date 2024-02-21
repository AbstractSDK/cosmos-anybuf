use crate::{
    types::neutron::tokenfactory::{DenomAuthorityMetadata, Params},
    StargateQuery, StargateQueryResponse,
};
use anybuf::{Anybuf, Bufany};

pub struct QueryParamsRequest {}

impl QueryParamsRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl StargateQuery for QueryParamsRequest {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.Query/Params"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().into_vec()
    }
}

pub struct QueryParamsResponse {
    pub params: Params, // 1
}

impl StargateQueryResponse for QueryParamsResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let params = deserialized.message(1)?;
        Some(Self {
            params: Params::from_bufany(params)?,
        })
    }
}

pub struct QueryDenomAuthorityMetadataRequest {
    pub creator: String,
    pub subdenom: String,
}

impl QueryDenomAuthorityMetadataRequest {
    pub fn new(creator: impl Into<String>, subdenom: impl Into<String>) -> Self {
        Self {
            creator: creator.into(),
            subdenom: subdenom.into(),
        }
    }
}

impl StargateQuery for QueryDenomAuthorityMetadataRequest {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.Query/DenomAuthorityMetadata"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.creator)
            .append_string(2, &self.subdenom)
            .into_vec()
    }
}

pub struct QueryDenomAuthorityMetadataResponse {
    pub authority_metadata: DenomAuthorityMetadata,
}

impl StargateQueryResponse for QueryDenomAuthorityMetadataResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let authority_metadata = deserialized.message(1)?;
        Some(Self {
            authority_metadata: DenomAuthorityMetadata::from_bufany(authority_metadata)?,
        })
    }
}

pub struct QueryDenomsFromCreatorRequest {
    pub creator: String,
}

impl QueryDenomsFromCreatorRequest {
    pub fn new(creator: impl Into<String>) -> Self {
        Self {
            creator: creator.into(),
        }
    }
}

impl StargateQuery for QueryDenomsFromCreatorRequest {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.Query/DenomsFromCreator"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_string(1, &self.creator).into_vec()
    }
}

pub struct QueryDenomsFromCreatorResponse {
    pub denoms: Vec<String>,
}

impl StargateQueryResponse for QueryDenomsFromCreatorResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let denoms = deserialized.repeated_string(1).ok()?;
        Some(Self { denoms })
    }
}

pub struct QueryBeforeSendHookAddressRequest {
    pub creator: String,
    pub subdenom: String,
}

impl QueryBeforeSendHookAddressRequest {
    pub fn new(creator: impl Into<String>, subdenom: impl Into<String>) -> Self {
        Self {
            creator: creator.into(),
            subdenom: subdenom.into(),
        }
    }
}

impl StargateQuery for QueryBeforeSendHookAddressRequest {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.Query/BeforeSendHookAddress"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.creator)
            .append_string(2, &self.subdenom)
            .into_vec()
    }
}

pub struct QueryBeforeSendHookAddressResponse {
    pub contract_addr: String,
}

impl StargateQueryResponse for QueryBeforeSendHookAddressResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let contract_addr = deserialized.string(1)?;
        Some(Self { contract_addr })
    }
}
