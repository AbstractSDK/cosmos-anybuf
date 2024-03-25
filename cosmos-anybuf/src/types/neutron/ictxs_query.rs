use crate::{types::neutron::interchainqueries::Params, StargateQuery, StargateResponse};
use anybuf::{Anybuf, Bufany};

pub struct QueryParamsRequest {}

impl QueryParamsRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl StargateQuery for QueryParamsRequest {
    fn url() -> &'static str {
        "/neutron.interchaintxs.v1.QueryParamsRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().into_vec()
    }
}

pub struct QueryParamsResponse {
    pub params: Params,
}

impl StargateResponse for QueryParamsResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let params = deserialized.message(1)?;
        Some(Self {
            params: Params::from_bufany(params)?,
        })
    }
}

pub struct QueryInterchainAccountAddressRequest {
    pub owner_address: String,
    pub interchain_account_id: String,
    pub connection_id: String,
}

impl QueryInterchainAccountAddressRequest {
    pub fn new(
        owner_address: impl Into<String>,
        interchain_account_id: impl Into<String>,
        connection_id: impl Into<String>,
    ) -> Self {
        Self {
            owner_address: owner_address.into(),
            interchain_account_id: interchain_account_id.into(),
            connection_id: connection_id.into(),
        }
    }
}

impl StargateQuery for QueryInterchainAccountAddressRequest {
    fn url() -> &'static str {
        "/neutron.interchaintxs.v1.QueryInterchainAccountAddressRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.owner_address)
            .append_string(2, &self.interchain_account_id)
            .append_string(3, &self.connection_id)
            .into_vec()
    }
}

pub struct QueryInterchainAccountAddressResponse {
    pub interchain_account_address: String,
}

impl StargateResponse for QueryInterchainAccountAddressResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let interchain_account_address = deserialized.string(1)?;
        Some(Self {
            interchain_account_address,
        })
    }
}
