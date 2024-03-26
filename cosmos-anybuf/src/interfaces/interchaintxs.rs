use cosmwasm_std::{CosmosMsg, QuerierWrapper, StdResult};

use crate::anybuf_interface::StargateResponse;
use crate::neutron::IbcFee;
use crate::Any;

pub trait InterchainTxs {
    // Params
    type Params;
    // Msg Responses
    type MsgSubmitTxResponse: StargateResponse;
    // Query Responses
    type QueryParamsResponse: StargateResponse;
    type QueryInterchainAccountAddressResponse: StargateResponse;

    fn register_interchain_account(
        from_address: impl Into<String>,
        connection_id: impl Into<String>,
        interchain_account_id: impl Into<String>,
        register_fee: Vec<cosmwasm_std::Coin>,
    ) -> CosmosMsg;

    fn submit_tx(
        from_address: impl Into<String>,
        interchain_account_id: impl Into<String>,
        connection_id: impl Into<String>,
        msgs: Vec<Any>,
        memo: String,
        timeout: u64,
        fee: IbcFee,
    ) -> CosmosMsg;

    fn parse_submit_tx_response(
        data: cosmwasm_std::Binary,
    ) -> StdResult<Self::MsgSubmitTxResponse> {
        Self::MsgSubmitTxResponse::from_buf(data.0).ok_or(cosmwasm_std::StdError::ParseErr {
            target_type: stringify!(MsgSubmitTxResponse).to_owned(),
            msg: "Failed to deserialize proto".to_owned(),
        })
    }

    fn update_params(authority: impl Into<String>, params: Self::Params) -> CosmosMsg;

    fn query_params(querier: &QuerierWrapper) -> StdResult<Self::QueryParamsResponse>;

    fn query_interchain_account_address(
        querier: &QuerierWrapper,
        owner_address: impl Into<String>,
        interchain_account_id: impl Into<String>,
        connection_id: impl Into<String>,
    ) -> StdResult<Self::QueryInterchainAccountAddressResponse>;
}
