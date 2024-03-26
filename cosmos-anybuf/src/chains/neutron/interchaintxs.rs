use cosmwasm_std::Coin;

use crate::{
    interfaces::InterchainTxs,
    types::neutron::{feerefunder, ictxs_query, ictxs_tx, interchaintxs},
    Any, StargateMsg,
};

use super::Neutron;

// Copied from https://github.com/neutron-org/neutron-sdk/blob/612ea5ac87d5760d2a4f6311ab5bdabd0bbbe5b4/packages/neutron-sdk/src/bindings/msg.rs#L20
/// IbcFee defines struct for fees that refund the relayer for `SudoMsg` messages submission.
/// Unused fee kind will be returned back to message sender.
/// Please refer to these links for more information:
/// IBC transaction structure - <https://docs.neutron.org/neutron/interchain-txs/messages/#msgsubmittx>
/// General mechanics of fee payments - <https://docs.neutron.org/neutron/feerefunder/overview/#general-mechanics>
#[cosmwasm_schema::cw_serde]
pub struct IbcFee {
    /// **recv_fee** currently is used for compatibility with ICS-29 interface only and must be set to zero (i.e. 0untrn),
    /// because Neutron's fee module can't refund relayer for submission of Recv IBC packets due to compatibility with target chains.
    pub recv_fee: Vec<Coin>,
    /// **ack_fee** is an amount of coins to refund relayer for submitting ack message for a particular IBC packet.
    pub ack_fee: Vec<Coin>,
    /// **timeout_fee** amount of coins to refund relayer for submitting timeout message for a particular IBC packet.
    pub timeout_fee: Vec<Coin>,
}

impl From<IbcFee> for feerefunder::Fee {
    fn from(value: IbcFee) -> Self {
        let IbcFee {
            recv_fee,
            ack_fee,
            timeout_fee,
        } = value;
        Self {
            recv_fee: recv_fee.into_iter().map(Into::into).collect(),
            ack_fee: ack_fee.into_iter().map(Into::into).collect(),
            timeout_fee: timeout_fee.into_iter().map(Into::into).collect(),
        }
    }
}

impl InterchainTxs for Neutron {
    type Params = interchaintxs::Params;
    type MsgSubmitTxResponse = ictxs_tx::MsgSubmitTxResponse;
    type QueryParamsResponse = ictxs_query::QueryParamsResponse;
    type QueryInterchainAccountAddressResponse = ictxs_query::QueryInterchainAccountAddressResponse;

    fn register_interchain_account(
        from_address: impl Into<String>,
        connection_id: impl Into<String>,
        interchain_account_id: impl Into<String>,
        register_fee: Vec<cosmwasm_std::Coin>,
    ) -> cosmwasm_std::CosmosMsg {
        let register_fee = register_fee.into_iter().map(Into::into).collect();
        ictxs_tx::MsgRegisterInterchainAccount {
            from_address: from_address.into(),
            connection_id: connection_id.into(),
            interchain_account_id: interchain_account_id.into(),
            register_fee,
        }
        .to_msg()
    }

    fn submit_tx(
        from_address: impl Into<String>,
        interchain_account_id: impl Into<String>,
        connection_id: impl Into<String>,
        msgs: Vec<Any>,
        memo: String,
        timeout: u64,
        fee: IbcFee,
    ) -> cosmwasm_std::CosmosMsg {
        ictxs_tx::MsgSubmitTx {
            from_address: from_address.into(),
            interchain_account_id: interchain_account_id.into(),
            connection_id: connection_id.into(),
            msgs,
            memo,
            timeout,
            fee: fee.into(),
        }
        .to_msg()
    }

    fn update_params(
        authority: impl Into<String>,
        params: Self::Params,
    ) -> cosmwasm_std::CosmosMsg {
        ictxs_tx::MsgUpdateParams {
            authority: authority.into(),
            params,
        }
        .to_msg()
    }

    fn query_params(
        querier: &cosmwasm_std::QuerierWrapper,
    ) -> cosmwasm_std::StdResult<Self::QueryParamsResponse> {
        crate::utils::query_decode(querier, ictxs_query::QueryParamsRequest {})
    }

    fn query_interchain_account_address(
        querier: &cosmwasm_std::QuerierWrapper,
        owner_address: impl Into<String>,
        interchain_account_id: impl Into<String>,
        connection_id: impl Into<String>,
    ) -> cosmwasm_std::StdResult<Self::QueryInterchainAccountAddressResponse> {
        crate::utils::query_decode(
            querier,
            ictxs_query::QueryInterchainAccountAddressRequest {
                owner_address: owner_address.into(),
                interchain_account_id: interchain_account_id.into(),
                connection_id: connection_id.into(),
            },
        )
    }
}
