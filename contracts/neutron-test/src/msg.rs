use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, DenomMetadata};

#[cw_serde]
pub enum ExecuteMsg {
    // tokenfactory
    CreateDenom {
        subdenom: String,
    },
    Mint {
        amount: Coin,
        mint_to_address: String,
    },
    Burn {
        amount: Coin,
        burn_from_address: String,
    },
    ChangeAdmin {
        denom: String,
        new_admin: String,
    },
    SetDenomMetadata {
        metadata: DenomMetadata,
    },
    SetBeforeSendHook {
        denom: String,
        contract_addr: String,
    },
    ForceTransfer {
        amount: Coin,
        transfer_from_address: String,
        transfer_to_address: String,
    },
    UpdateParams {
        authority: String,
        params: Binary,
    },
}

#[cw_serde]
pub enum QueryMsg {
    // tokenfactory
    DenomAuthorityMetadata { creator: String, subdenom: String },
    Params {},
    DenomsFromCreator { creator: String },
    BeforeSendHookAddress { creator: String, subdenom: String },
}
