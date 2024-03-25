use cosmwasm_std::{CosmosMsg, QuerierWrapper, StdResult};

use crate::anybuf_interface::StargateResponse;
use crate::types::query::PageRequest;

use crate::types::neutron::interchainqueries::KVKey;
pub trait InterChainQueries {
    type Params;
    type MsgRegisterInterchainQueryResponse: StargateResponse;
    type QueryParamsResponse: StargateResponse;
    type QueryRegisteredQueriesResponse: StargateResponse;
    type QueryRegisteredQueryResponse: StargateResponse;
    type QueryRegisteredQueryResultResponse: StargateResponse;
    type QueryLastRemoteHeightResponse: StargateResponse;

    fn register_interchain_query(
        sender: impl Into<String>,
        query_type: impl Into<String>,
        keys: Vec<KVKey>,
        transactions_filter: impl Into<String>,
        connection_id: impl Into<String>,
        update_period: u64,
    ) -> CosmosMsg;

    fn parse_register_interchain_query_response(
        data: cosmwasm_std::Binary,
    ) -> StdResult<Self::MsgRegisterInterchainQueryResponse> {
        Self::MsgRegisterInterchainQueryResponse::from_buf(data.0).ok_or(cosmwasm_std::StdError::ParseErr {
            target_type: stringify!(InterchainQueryResponse).to_owned(),
            msg: "Failed to deserialize proto".to_owned(),
        })
    }

    fn remove_interchain_query(sender: impl Into<String>, query_id: u64) -> CosmosMsg;

    fn update_interchain_query(
        sender: impl Into<String>,
        query_id: u64,
        new_keys: Vec<KVKey>,
        new_update_period: u64,
        new_transactions_filter: String,
    ) -> CosmosMsg;

    fn update_params(authority: impl Into<String>, params: Self::Params) -> CosmosMsg;

    fn query_params(querier: &QuerierWrapper) -> StdResult<Self::QueryParamsResponse>;

    fn query_registered_queries(
        querier: &QuerierWrapper,
        owners: Vec<impl Into<String>>,
        connection_id: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<Self::QueryRegisteredQueriesResponse>;

    fn query_registered_query(
        querier: &QuerierWrapper,
        query_id: u64
    ) -> StdResult<Self::QueryRegisteredQueryResponse>;

    fn query_registered_query_result(
        querier: &QuerierWrapper,
        query_id: u64
    ) -> StdResult<Self::QueryRegisteredQueryResultResponse>;

    fn query_last_remote_height(
        querier: &QuerierWrapper,
connection_id: impl Into<String>,
    ) -> StdResult<Self::QueryLastRemoteHeightResponse>;
}
