use cosmwasm_std::{CosmosMsg, QuerierWrapper, StdResult};

use crate::{anybuf_interface::StargateResponse, types::bank::Metadata};

pub trait TokenFactory {
    // Params
    type Params;
    // Msg responses
    type MsgCreateDenomResponse: StargateResponse;
    // Query Responses
    type QueryDenomAuthorityMetadataResponse: StargateResponse;
    type QueryParamsResponse: StargateResponse;
    type QueryDenomsFromCreatorResponse: StargateResponse;
    type QueryBeforeSendHookAddressResponse: StargateResponse;

    fn create_denom(sender: impl Into<String>, subdenom: impl Into<String>) -> CosmosMsg;

    fn parse_create_denom_response(
        data: cosmwasm_std::Binary,
    ) -> StdResult<Self::MsgCreateDenomResponse> {
        Self::MsgCreateDenomResponse::from_buf(data.0).ok_or(cosmwasm_std::StdError::ParseErr {
            target_type: stringify!(MsgCreateDenomResponse).to_owned(),
            msg: "Failed to deserialize proto".to_owned(),
        })
    }

    fn mint(
        sender: impl Into<String>,
        amount: cosmwasm_std::Coin,
        mint_to_address: impl Into<String>,
    ) -> CosmosMsg;

    fn burn(
        sender: impl Into<String>,
        amount: cosmwasm_std::Coin,
        burn_from_address: impl Into<String>,
    ) -> CosmosMsg;

    fn change_admin(
        sender: impl Into<String>,
        denom: impl Into<String>,
        new_admin: impl Into<String>,
    ) -> CosmosMsg;

    fn set_denom_metadata(sender: impl Into<String>, metadata: Metadata) -> CosmosMsg;

    fn set_before_send_hook(
        sender: impl Into<String>,
        denom: impl Into<String>,
        contract_addr: impl Into<String>,
    ) -> CosmosMsg;

    fn force_transfer(
        sender: impl Into<String>,
        amount: cosmwasm_std::Coin,
        transfer_from_address: impl Into<String>,
        transfer_to_address: impl Into<String>,
    ) -> CosmosMsg;

    fn update_params(authority: impl Into<String>, params: Self::Params) -> CosmosMsg;

    fn query_denom_authority_metadata(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: impl Into<String>,
        subdenom: impl Into<String>,
    ) -> StdResult<Self::QueryDenomAuthorityMetadataResponse>;

    fn query_params(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
    ) -> StdResult<Self::QueryParamsResponse>;

    fn query_denoms_from_creator(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: impl Into<String>,
    ) -> StdResult<Self::QueryDenomsFromCreatorResponse>;

    fn query_before_send_hook_address_request(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: impl Into<String>,
        subdenom: impl Into<String>,
    ) -> StdResult<Self::QueryBeforeSendHookAddressResponse>;
}
