use cosmwasm_schema::serde::{Serialize, Serializer};
use cosmwasm_std::{CosmosMsg, QuerierWrapper, StdResult};

use crate::anybuf_interface::StargateResponse;
use crate::types::query::PageRequest;

use crate::types::neutron::interchainqueries::KVKey;

// copied from https://github.com/neutron-org/neutron-sdk/blob/612ea5ac87d5760d2a4f6311ab5bdabd0bbbe5b4/packages/neutron-sdk/src/interchain_queries/types.rs#L16
// Removed Deserialize for now
#[derive(Serialize, PartialEq, Eq, Debug)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub enum TransactionFilterOp {
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TransactionFilterValue {
    String(String),
    Int(u64),
}

impl Serialize for TransactionFilterValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TransactionFilterValue::String(v) => serializer.serialize_str(v),
            TransactionFilterValue::Int(v) => serializer.serialize_u64(*v),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "::cosmwasm_schema::serde")]
pub struct TransactionFilterItem {
    pub field: String,
    pub op: TransactionFilterOp,
    pub value: TransactionFilterValue,
}

pub trait InterChainQueries {
    type Params;
    type MsgRegisterInterchainQueryResponse: StargateResponse;
    type QueryParamsResponse: StargateResponse;
    type QueryRegisteredQueriesResponse: StargateResponse;
    type QueryRegisteredQueryResponse: StargateResponse;
    type QueryRegisteredQueryResultResponse: StargateResponse;
    type QueryLastRemoteHeightResponse: StargateResponse;

    /// For Transactions Filter you can use helper type [`TransactionFilterItem`]
    /// For example:
    ///
    /// cosmwasm_std::to_json_string(&transactions_filter)
    ///
    /// Where transactions_filter: Vec<[`TransactionFilterItem`]>
    fn register_interchain_query(
        sender: impl Into<String>,
        query_type: impl Into<String>,
        keys: Vec<KVKey>,
        transactions_filter: String,
        connection_id: impl Into<String>,
        update_period: u64,
    ) -> CosmosMsg;

    fn parse_register_interchain_query_response(
        data: cosmwasm_std::Binary,
    ) -> StdResult<Self::MsgRegisterInterchainQueryResponse> {
        Self::MsgRegisterInterchainQueryResponse::from_buf(data.0).ok_or(
            cosmwasm_std::StdError::ParseErr {
                target_type: stringify!(InterchainQueryResponse).to_owned(),
                msg: "Failed to deserialize proto".to_owned(),
            },
        )
    }

    fn remove_interchain_query(sender: impl Into<String>, query_id: u64) -> CosmosMsg;

    /// For Transactions Filter you can use helper type [`TransactionFilterItem`]
    /// For example:
    ///
    /// cosmwasm_std::to_json_string(&transactions_filter)
    ///
    /// Where transactions_filter: Vec<[`TransactionFilterItem`]>
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
        query_id: u64,
    ) -> StdResult<Self::QueryRegisteredQueryResponse>;

    fn query_registered_query_result(
        querier: &QuerierWrapper,
        query_id: u64,
    ) -> StdResult<Self::QueryRegisteredQueryResultResponse>;

    fn query_last_remote_height(
        querier: &QuerierWrapper,
        connection_id: impl Into<String>,
    ) -> StdResult<Self::QueryLastRemoteHeightResponse>;
}
