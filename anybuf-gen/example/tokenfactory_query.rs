use crate::{
    osmosis::tokenfactory::{DenomAuthorityMetadata, Params},
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
        "/osmosis.tokenfactory.v1beta1.QueryParamsRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().into_vec()
    }
}

pub struct QueryParamsResponse {
    pub params: Params,
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
    pub denom: String,
}

impl QueryDenomAuthorityMetadataRequest {
    pub fn new(denom: impl Into<String>) -> Self {
        Self {
            denom: denom.into(),
        }
    }
}

impl StargateQuery for QueryDenomAuthorityMetadataRequest {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_string(1, &self.denom).into_vec()
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
