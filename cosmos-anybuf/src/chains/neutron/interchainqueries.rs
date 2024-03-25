use cosmwasm_schema::serde::{Serialize, Serializer};
use cosmwasm_std::to_json_string;

use crate::interfaces::InterChainQueries;

use crate::types::neutron::{icq_query, icq_tx, interchainqueries};
use crate::StargateMsg;

use super::Neutron;

impl InterChainQueries for Neutron {
    type Params = interchainqueries::Params;
    type MsgRegisterInterchainQueryResponse = icq_tx::MsgRegisterInterchainQueryResponse;
    type QueryParamsResponse = icq_query::QueryParamsResponse;
    type QueryRegisteredQueriesResponse = icq_query::QueryRegisteredQueriesResponse;
    type QueryRegisteredQueryResponse = icq_query::QueryRegisteredQueryResponse;
    type QueryRegisteredQueryResultResponse = icq_query::QueryRegisteredQueryResultResponse;
    type QueryLastRemoteHeightResponse = icq_query::QueryLastRemoteHeightResponse;

    fn register_interchain_query(
        sender: impl Into<String>,
        query_type: impl Into<String>,
        keys: Vec<crate::types::neutron::interchainqueries::KVKey>,
        transactions_filter: String,
        connection_id: impl Into<String>,
        update_period: u64,
    ) -> cosmwasm_std::CosmosMsg {
        icq_tx::MsgRegisterInterchainQuery {
            query_type: query_type.into(),
            keys,
            transactions_filter,
            connection_id: connection_id.into(),
            update_period,
            sender: sender.into(),
        }
        .to_msg()
    }

    fn remove_interchain_query(
        sender: impl Into<String>,
        query_id: u64,
    ) -> cosmwasm_std::CosmosMsg {
        icq_tx::MsgRemoveInterchainQueryRequest {
            query_id,
            sender: sender.into(),
        }
        .to_msg()
    }

    fn update_interchain_query(
        sender: impl Into<String>,
        query_id: u64,
        new_keys: Vec<crate::types::neutron::interchainqueries::KVKey>,
        new_update_period: u64,
        new_transactions_filter: String,
    ) -> cosmwasm_std::CosmosMsg {
        icq_tx::MsgUpdateInterchainQueryRequest {
            query_id: query_id.into(),
            new_keys,
            new_update_period,
            new_transactions_filter,
            sender: sender.into(),
        }
        .to_msg()
    }

    fn update_params(
        authority: impl Into<String>,
        params: Self::Params,
    ) -> cosmwasm_std::CosmosMsg {
        icq_tx::MsgUpdateParams {
            authority: authority.into(),
            params,
        }
        .to_msg()
    }

    fn query_params(
        querier: &cosmwasm_std::QuerierWrapper,
    ) -> cosmwasm_std::StdResult<Self::QueryParamsResponse> {
        crate::utils::query_decode(querier, icq_query::QueryParamsRequest {})
    }

    fn query_registered_queries(
        querier: &cosmwasm_std::QuerierWrapper,
        owners: Vec<impl Into<String>>,
        connection_id: impl Into<String>,
        pagination: Option<crate::query::PageRequest>,
    ) -> cosmwasm_std::StdResult<Self::QueryRegisteredQueriesResponse> {
        let owners = owners.into_iter().map(Into::into).collect();
        crate::utils::query_decode(
            querier,
            icq_query::QueryRegisteredQueriesRequest {
                owners,
                connection_id: connection_id.into(),
                pagination,
            },
        )
    }

    fn query_registered_query(
        querier: &cosmwasm_std::QuerierWrapper,
        query_id: u64,
    ) -> cosmwasm_std::StdResult<Self::QueryRegisteredQueryResponse> {
        crate::utils::query_decode(querier, icq_query::QueryRegisteredQueryRequest { query_id })
    }

    fn query_registered_query_result(
        querier: &cosmwasm_std::QuerierWrapper,
        query_id: u64,
    ) -> cosmwasm_std::StdResult<Self::QueryRegisteredQueryResultResponse> {
        crate::utils::query_decode(
            querier,
            icq_query::QueryRegisteredQueryResultRequest { query_id },
        )
    }

    fn query_last_remote_height(
        querier: &cosmwasm_std::QuerierWrapper,
        connection_id: impl Into<String>,
    ) -> cosmwasm_std::StdResult<Self::QueryLastRemoteHeightResponse> {
        crate::utils::query_decode(
            querier,
            icq_query::QueryLastRemoteHeight {
                connection_id: connection_id.into(),
            },
        )
    }
}
