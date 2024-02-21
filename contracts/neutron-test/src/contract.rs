use cosmos_anybuf::{chains::neutron::Neutron, interfaces::TokenFactory};
use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdResult,
};

use crate::msg::{ExecuteMsg, QueryMsg};

// A no-op, just empty data
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: cosmwasm_std::Empty,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let sender = env.contract.address.to_string();
    let message = match msg {
        ExecuteMsg::CreateDenom { subdenom } => Neutron::create_denom(sender, subdenom),
        ExecuteMsg::Mint {
            amount,
            mint_to_address,
        } => Neutron::mint(sender, amount, mint_to_address),
        ExecuteMsg::Burn {
            amount,
            burn_from_address,
        } => Neutron::burn(sender, amount, burn_from_address),
        ExecuteMsg::ChangeAdmin { denom, new_admin } => {
            Neutron::change_admin(sender, denom, new_admin)
        }
        ExecuteMsg::SetDenomMetadata { metadata } => {
            Neutron::set_denom_metadata(sender, metadata.into())
        }
        ExecuteMsg::SetBeforeSendHook {
            denom,
            contract_addr,
        } => Neutron::set_before_send_hook(sender, denom, contract_addr),
        ExecuteMsg::ForceTransfer {
            amount,
            transfer_from_address,
            transfer_to_address,
        } => Neutron::force_transfer(sender, amount, transfer_from_address, transfer_to_address),
        ExecuteMsg::UpdateParams { authority, params } => Neutron::update_params(
            authority,
            <Neutron as TokenFactory>::Params::from_buf(params.0).unwrap(),
        ),
    };
    Ok(Response::new().add_message(message))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::DenomAuthorityMetadata { creator, subdenom } => to_json_binary(
            &Neutron::query_denom_authority_metadata(&deps.querier, creator, subdenom)?
                .authority_metadata
                .admin,
        ),
        QueryMsg::Params {} => to_json_binary(
            &Neutron::query_params(&deps.querier)?
                .params
                .denom_creation_gas_consume,
        ),
        QueryMsg::DenomsFromCreator { creator } => {
            to_json_binary(&Neutron::query_denoms_from_creator(&deps.querier, creator)?.denoms)
        }
        QueryMsg::BeforeSendHookAddress { creator, subdenom } => to_json_binary(
            &Neutron::query_before_send_hook_address_request(&deps.querier, creator, subdenom)?
                .contract_addr,
        ),
    }
}
