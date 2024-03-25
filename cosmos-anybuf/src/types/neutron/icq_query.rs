use crate::{
    types::query::{PageRequest, PageResponse},
    StargateQuery, StargateResponse,
};
use anybuf::{Anybuf, Bufany};

use super::interchainqueries::{Params, QueryResult, RegisteredQuery};

pub struct QueryParamsRequest;

impl StargateQuery for QueryParamsRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/Params"
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

pub struct QueryRegisteredQueriesRequest {
    pub owners: Vec<String>,             // 1
    pub connection_id: String,           // 2
    pub pagination: Option<PageRequest>, // 3
}

impl StargateQuery for QueryRegisteredQueriesRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/RegisteredQueries"
    }

    fn to_buf(&self) -> Vec<u8> {
        let pagination_message = self
            .pagination
            .as_ref()
            .map(PageRequest::to_anybuf)
            .unwrap_or_default();
        Anybuf::new()
            .append_repeated_string(1, &self.owners)
            .append_string(2, &self.connection_id)
            .append_message(3, &pagination_message)
            .into_vec()
    }
}

pub struct QueryRegisteredQueriesResponse {
    pub registered_queries: Vec<RegisteredQuery>, // 1
    pub pagination: Option<PageResponse>,         // 2
}

impl StargateResponse for QueryRegisteredQueriesResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let registered_queries_bytes = deserialized.repeated_bytes(1)?;
        let registered_queries = registered_queries_bytes
            .into_iter()
            .map(RegisteredQuery::from_buf)
            .collect::<Option<_>>()?;
        let pagination_buf = deserialized.message(2);
        let pagination = pagination_buf.map(PageResponse::from_bufany).flatten();
        Some(Self {
            registered_queries,
            pagination,
        })
    }
}

pub struct QueryRegisteredQueryRequest {
    pub query_id: u64, // 1
}

impl StargateQuery for QueryRegisteredQueryRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/RegisteredQuery"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_uint64(1, self.query_id).into_vec()
    }
}

pub struct QueryRegisteredQueryResponse {
    pub registered_query: RegisteredQuery, // 1
}

impl StargateResponse for QueryRegisteredQueryResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let bufany = Bufany::deserialize(&buf).ok()?;
        let registered_query = RegisteredQuery::from_bufany(bufany)?;
        Some(Self { registered_query })
    }
}

pub struct QueryRegisteredQueryResultRequest {
    pub query_id: u64, // 1
}

impl StargateQuery for QueryRegisteredQueryResultRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/QueryResult"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_uint64(1, self.query_id).into_vec()
    }
}

pub struct QueryRegisteredQueryResultResponse {
    pub result: QueryResult, // 1
}

impl StargateResponse for QueryRegisteredQueryResultResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let buf = Bufany::deserialize(&buf).ok()?;
        let result_message = buf.message(1)?;
        let result = QueryResult::from_bufany(result_message)?;
        Some(Self { result })
    }
}

pub struct QueryLastRemoteHeight {
    pub connection_id: String, // 1
}

impl StargateQuery for QueryLastRemoteHeight {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/LastRemoteHeight"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.connection_id)
            .into_vec()
    }
}

pub struct QueryLastRemoteHeightResponse {
    pub height: u64, // 1
}

impl StargateResponse for QueryLastRemoteHeightResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let height = deserialized.uint64(1)?;
        Some(Self { height })
    }
}
