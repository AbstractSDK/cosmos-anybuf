use anybuf::{Anybuf, Bufany};

use crate::{StargateQuery, StargateResponse};

use super::feerefunder::Params;

pub struct QueryParamsRequest {}

impl QueryParamsRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl StargateQuery for QueryParamsRequest {
    fn url() -> &'static str {
        "/neutron.feerefunder.QueryParamsRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().into_vec()
    }
}

pub struct QueryParamsResponse {
    pub params: Params, // 1
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
