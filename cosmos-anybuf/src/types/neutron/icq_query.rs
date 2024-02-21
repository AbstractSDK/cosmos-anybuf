use anybuf::Anybuf;
use crate::{StargateQuery, StargateQueryResponse, Params, RegisteredQuery, QueryResult, PageRequest, PageResponse};

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
    pub params: Params,
}

impl StargateQueryResponse for QueryParamsResponse {
    fn from_buf(buf: Vec<u8>) -> Self {
        let anybuf = Anybuf::from(buf);
        Self {
            params: Params::from_buf(anybuf.get_bytes(1).unwrap()),
        }
    }
}

pub struct QueryRegisteredQueriesRequest {
    pub owners: Vec<String>,
    pub connection_id: String,
    pub pagination: Option<PageRequest>,
}

impl StargateQuery for QueryRegisteredQueriesRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/RegisteredQueries"
    }

    fn to_buf(&self) -> Vec<u8> {
        let mut anybuf = Anybuf::new();
        for owner in &self.owners {
            anybuf.append_string(1, owner);
        }
        anybuf.append_string(2, &self.connection_id);
        if let Some(pagination) = &self.pagination {
            anybuf.append_bytes(3, &pagination.to_buf());
        }
        anybuf.into_vec()
    }
}

pub struct QueryRegisteredQueriesResponse {
    pub registered_queries: Vec<RegisteredQuery>,
    pub pagination: Option<PageResponse>,
}

impl StargateQueryResponse for QueryRegisteredQueriesResponse {
    fn from_buf(buf: Vec<u8>) -> Self {
        let anybuf = Anybuf::from(buf);
        Self {
            registered_queries: anybuf.get_repeated_bytes(1).unwrap().iter().map(|b| RegisteredQuery::from_buf(b.to_vec())).collect(),
            pagination: anybuf.get_bytes(2).map(|b| PageResponse::from_buf(b.to_vec())),
        }
    }
}

pub struct QueryRegisteredQueryRequest {
    pub query_id: u64,
}

impl StargateQuery for QueryRegisteredQueryRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/RegisteredQuery"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_u64(1, self.query_id).into_vec()
    }
}

pub struct QueryRegisteredQueryResponse {
    pub registered_query: RegisteredQuery,
}

impl StargateQueryResponse for QueryRegisteredQueryResponse {
    fn from_buf(buf: Vec<u8>) -> Self {
        let anybuf = Anybuf::from(buf);
        Self {
            registered_query: RegisteredQuery::from_buf(anybuf.get_bytes(1).unwrap()),
        }
    }
}

pub struct QueryRegisteredQueryResultRequest {
    pub query_id: u64,
}

impl StargateQuery for QueryRegisteredQueryResultRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/QueryResult"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_u64(1, self.query_id).into_vec()
    }
}

pub struct QueryRegisteredQueryResultResponse {
    pub result: QueryResult,
}

impl StargateQueryResponse for QueryRegisteredQueryResultResponse {
    fn from_buf(buf: Vec<u8>) -> Self {
        let anybuf = Anybuf::from(buf);
        Self {
            result: QueryResult::from_buf(anybuf.get_bytes(1).unwrap()),
        }
    }
}

pub struct QueryLastRemoteHeight {
    pub connection_id: String,
}

impl StargateQuery for QueryLastRemoteHeight {
    fn url() -> &'static str {
        "/neutron.interchainqueries.Query/LastRemoteHeight"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_string(1, &self.connection_id).into_vec()
    }
}

pub struct QueryLastRemoteHeightResponse {
    pub height: u64,
}

impl StargateQueryResponse for QueryLastRemoteHeightResponse {
    fn from_buf(buf: Vec<u8>) -> Self {
        let anybuf = Anybuf::from(buf);
        Self {
            height: anybuf.get_u64(1).unwrap(),
        }
    }
}
