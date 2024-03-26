use crate::anybuf_interface::StargateMsg;
use crate::interfaces::TokenFactory;
use crate::types::neutron::tokenfactory_query::{
    QueryBeforeSendHookAddressResponse, QueryDenomAuthorityMetadataResponse,
    QueryDenomsFromCreatorResponse, QueryParamsResponse,
};
use crate::types::neutron::{tokenfactory::Params, tokenfactory_query, tokenfactory_tx};

use cosmwasm_std::{QuerierWrapper, StdResult};

use super::Neutron;

impl TokenFactory for Neutron {
    type Params = Params;
    type MsgCreateDenomResponse = tokenfactory_tx::MsgCreateDenomResponse;
    type QueryDenomAuthorityMetadataResponse = QueryDenomAuthorityMetadataResponse;
    type QueryParamsResponse = QueryParamsResponse;
    type QueryDenomsFromCreatorResponse = QueryDenomsFromCreatorResponse;
    type QueryBeforeSendHookAddressResponse = QueryBeforeSendHookAddressResponse;

    fn create_denom(
        sender: impl Into<String>,
        subdenom: impl Into<String>,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgCreateDenom {
            sender: sender.into(),
            subdenom: subdenom.into(),
        }
        .to_msg()
    }

    fn mint(
        sender: impl Into<String>,
        amount: cosmwasm_std::Coin,
        mint_to_address: impl Into<String>,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgMint {
            sender: sender.into(),
            amount: amount.into(),
            mint_to_address: mint_to_address.into(),
        }
        .to_msg()
    }

    fn burn(
        sender: impl Into<String>,
        amount: cosmwasm_std::Coin,
        burn_from_address: impl Into<String>,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgBurn {
            sender: sender.into(),
            amount: amount.into(),
            burn_from_address: burn_from_address.into(),
        }
        .to_msg()
    }

    fn change_admin(
        sender: impl Into<String>,
        denom: impl Into<String>,
        new_admin: impl Into<String>,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgChangeAdmin {
            sender: sender.into(),
            denom: denom.into(),
            new_admin: new_admin.into(),
        }
        .to_msg()
    }

    fn set_denom_metadata(
        sender: impl Into<String>,
        metadata: crate::bank::Metadata,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgSetDenomMetadata {
            sender: sender.into(),
            metadata,
        }
        .to_msg()
    }

    fn set_before_send_hook(
        sender: impl Into<String>,
        denom: impl Into<String>,
        contract_addr: impl Into<String>,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgSetBeforeSendHook {
            sender: sender.into(),
            denom: denom.into(),
            contract_addr: contract_addr.into(),
        }
        .to_msg()
    }

    fn force_transfer(
        sender: impl Into<String>,
        amount: cosmwasm_std::Coin,
        transfer_from_address: impl Into<String>,
        transfer_to_address: impl Into<String>,
    ) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgForceTransfer {
            sender: sender.into(),
            amount: amount.into(),
            transfer_from_address: transfer_from_address.into(),
            transfer_to_address: transfer_to_address.into(),
        }
        .to_msg()
    }

    fn update_params(authority: impl Into<String>, params: Params) -> cosmwasm_std::CosmosMsg {
        tokenfactory_tx::MsgUpdateParams {
            authority: authority.into(),
            params,
        }
        .to_msg()
    }

    fn query_denom_authority_metadata(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: impl Into<String>,
        subdenom: impl Into<String>,
    ) -> StdResult<QueryDenomAuthorityMetadataResponse> {
        crate::utils::query_decode(
            querier,
            tokenfactory_query::QueryDenomAuthorityMetadataRequest {
                creator: creator.into(),
                subdenom: subdenom.into(),
            },
        )
    }

    fn query_params(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
    ) -> StdResult<QueryParamsResponse> {
        crate::utils::query_decode(querier, tokenfactory_query::QueryParamsRequest {})
    }

    fn query_denoms_from_creator(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: impl Into<String>,
    ) -> StdResult<QueryDenomsFromCreatorResponse> {
        crate::utils::query_decode(
            querier,
            tokenfactory_query::QueryDenomsFromCreatorRequest {
                creator: creator.into(),
            },
        )
    }

    fn query_before_send_hook_address_request(
        querier: &QuerierWrapper<cosmwasm_std::Empty>,
        creator: impl Into<String>,
        subdenom: impl Into<String>,
    ) -> StdResult<QueryBeforeSendHookAddressResponse> {
        crate::utils::query_decode(
            querier,
            tokenfactory_query::QueryBeforeSendHookAddressRequest {
                creator: creator.into(),
                subdenom: subdenom.into(),
            },
        )
    }
}
