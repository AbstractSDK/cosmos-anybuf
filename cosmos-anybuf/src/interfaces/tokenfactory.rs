use cosmwasm_std::{CosmosMsg, QuerierWrapper, StdResult};

use crate::{anybuf_interface::StargateQueryResponse, types::bank::Metadata};

pub trait TokenFactory {
    type Params;
    type QueryDenomAuthorityMetadataResponse: StargateQueryResponse;
    type QueryParamsResponse: StargateQueryResponse;
    type QueryDenomsFromCreatorResponse: StargateQueryResponse;
    type QueryBeforeSendHookAddressResponse: StargateQueryResponse;

    fn create_denom<S: Into<String>>(sender: S, subdenom: S) -> CosmosMsg;

    fn mint<S: Into<String>>(
        sender: S,
        amount: cosmwasm_std::Coin,
        mint_to_address: S,
    ) -> CosmosMsg;

    fn burn<S: Into<String>>(
        sender: S,
        amount: cosmwasm_std::Coin,
        burn_from_address: S,
    ) -> CosmosMsg;

    fn change_admin<S: Into<String>>(sender: S, denom: S, new_admin: S) -> CosmosMsg;

    fn set_denom_metadata<S: Into<String>>(sender: S, metadata: Metadata) -> CosmosMsg;

    fn set_before_send_hook<S: Into<String>>(sender: S, denom: S, contract_addr: S) -> CosmosMsg;

    fn force_transfer<S: Into<String>>(
        sender: S,
        amount: cosmwasm_std::Coin,
        transfer_from_address: S,
        transfer_to_address: S,
    ) -> CosmosMsg;

    fn update_params<S: Into<String>>(authority: S, params: Self::Params) -> CosmosMsg;

    fn query_denom_authority_metadata<S: Into<String>>(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: S,
        subdenom: S,
    ) -> StdResult<Self::QueryDenomAuthorityMetadataResponse>;

    fn query_params(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
    ) -> StdResult<Self::QueryParamsResponse>;

    fn query_denoms_from_creator<S: Into<String>>(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: S,
    ) -> StdResult<Self::QueryDenomsFromCreatorResponse>;

    fn query_before_send_hook_address_request<S: Into<String>>(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: S,
        subdenom: S,
    ) -> StdResult<Self::QueryBeforeSendHookAddressResponse>;
}
