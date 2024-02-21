use crate::{
    cosmos::base::query::v1beta1::PageRequest,
    neutron::dex::{
        DepositRecord, LimitOrderTranche, LimitOrderTrancheUser, Params, Pool, PoolMetadata,
        PoolReserves, TickLiquidity,
    },
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
        "/neutron.dex.v1.QueryParamsRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().into_vec()
    }
}

pub struct QueryParamsResponse {
    pub params: Params,
}

impl StargateQueryResponse for QueryParamsResponse {
    fn url() -> &'static str {
        "/neutron.dex.v1.QueryParamsResponse"
    }

    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let params = deserialized.message(1)?;
        Some(Self {
            params: Params::from_bufany(params)?,
        })
    }
}

// The following structs and impls are generated for each Query and Response pair
// The actual fields and methods would need to be implemented according to the .proto definitions

// Example for QueryGetLimitOrderTrancheUserRequest
pub struct QueryGetLimitOrderTrancheUserRequest {
    pub address: String,
    pub tranche_key: String,
}

impl QueryGetLimitOrderTrancheUserRequest {
    pub fn new(address: impl Into<String>, tranche_key: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            tranche_key: tranche_key.into(),
        }
    }
}

impl StargateQuery for QueryGetLimitOrderTrancheUserRequest {
    fn url() -> &'static str {
        "/neutron.dex.v1.QueryGetLimitOrderTrancheUserRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.address)
            .append_string(2, &self.tranche_key)
            .into_vec()
    }
}

pub struct QueryGetLimitOrderTrancheUserResponse {
    pub limit_order_tranche_user: LimitOrderTrancheUser,
}

impl StargateQueryResponse for QueryGetLimitOrderTrancheUserResponse {
    fn url() -> &'static str {
        "/neutron.dex.v1.QueryGetLimitOrderTrancheUserResponse"
    }

    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let limit_order_tranche_user = deserialized.message(1)?;
        Some(Self {
            limit_order_tranche_user: LimitOrderTrancheUser::from_bufany(limit_order_tranche_user)?,
        })
    }
}

// The rest of the structs and impls would follow the same pattern as above
// ...
