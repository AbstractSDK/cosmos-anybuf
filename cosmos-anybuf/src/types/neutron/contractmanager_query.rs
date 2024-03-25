use crate::{
    types::{
        neutron::contractmanager::{Failure, Params},
        query::{PageRequest, PageResponse},
    },
    StargateQuery, StargateResponse,
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
        "/neutron.contractmanager.QueryParamsRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().into_vec()
    }
}

pub struct QueryParamsResponse {
    pub params: Params,
}

impl QueryParamsResponse {
    pub fn new(params: Params) -> Self {
        Self { params }
    }
}

impl StargateResponse for QueryParamsResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let message = deserialized.message(1)?;
        let params = Params::from_bufany(message)?;
        Some(Self { params })
    }
}

pub struct QueryFailuresRequest {
    pub address: String,
    pub failure_id: u64,
    pub pagination: Option<PageRequest>,
}

impl QueryFailuresRequest {
    pub fn new(
        address: impl Into<String>,
        failure_id: u64,
        pagination: Option<PageRequest>,
    ) -> Self {
        Self {
            address: address.into(),
            failure_id,
            pagination,
        }
    }
}

impl StargateQuery for QueryFailuresRequest {
    fn url() -> &'static str {
        "/neutron.contractmanager.QueryFailuresRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        let mut buf = Anybuf::new();
        buf = buf.append_string(1, &self.address);
        buf = buf.append_uint64(2, self.failure_id);
        if let Some(pagination) = &self.pagination {
            buf = buf.append_message(3, &pagination.to_anybuf());
        }
        buf.into_vec()
    }
}

pub struct QueryFailuresResponse {
    pub failures: Vec<Failure>,
    pub pagination: Option<PageResponse>,
}

impl QueryFailuresResponse {
    pub fn new(failures: Vec<Failure>, pagination: Option<PageResponse>) -> Self {
        Self {
            failures,
            pagination,
        }
    }
}

impl StargateResponse for QueryFailuresResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let failures = deserialized
            .repeated_bytes(1)?
            .into_iter()
            .filter_map(Failure::from_buf)
            .collect();
        let pagination = deserialized
            .message(2)
            .map(PageResponse::from_bufany)
            .flatten();
        Some(Self {
            failures,
            pagination,
        })
    }
}
