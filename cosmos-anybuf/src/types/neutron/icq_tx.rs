use crate::{
    types::{bank::Metadata, coin::Coin},
    StargateMsg,
};
use anybuf::Anybuf;

pub struct MsgCreateDenom {
    pub sender: String,
    pub subdenom: String,
}

impl MsgCreateDenom {
    pub fn new(sender: impl Into<String>, subdenom: impl Into<String>) -> Self {
        Self {
            sender: sender.into(),
            subdenom: subdenom.into(),
        }
    }
}

impl StargateMsg for MsgCreateDenom {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgCreateDenom"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_string(2, &self.subdenom)
            .into_vec()
    }
}

pub struct MsgCreateDenomResponse {
    pub new_token_denom: String,
}

impl MsgCreateDenomResponse {
    pub fn new(new_token_denom: impl Into<String>) -> Self {
        Self {
            new_token_denom: new_token_denom.into(),
        }
    }
}

pub struct MsgMint {
    pub sender: String,
    pub amount: Coin,
    pub mint_to_address: String,
}

impl MsgMint {
    pub fn new(
        sender: impl Into<String>,
        amount: Coin,
        mint_to_address: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            amount,
            mint_to_address: mint_to_address.into(),
        }
    }
}

impl StargateMsg for MsgMint {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgMint"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_bytes(2, self.amount.to_buf())
            .append_string(3, &self.mint_to_address)
            .into_vec()
    }
}

pub struct MsgMintResponse;

pub struct MsgBurn {
    pub sender: String,
    pub amount: Coin,
    pub burn_from_address: String,
}

impl MsgBurn {
    pub fn new(
        sender: impl Into<String>,
        amount: Coin,
        burn_from_address: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            amount,
            burn_from_address: burn_from_address.into(),
        }
    }
}

impl StargateMsg for MsgBurn {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgBurn"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_bytes(2, self.amount.to_buf())
            .append_string(3, &self.burn_from_address)
            .into_vec()
    }
}

pub struct MsgBurnResponse;

pub struct MsgChangeAdmin {
    pub sender: String,
    pub denom: String,
    pub new_admin: String,
}

impl MsgChangeAdmin {
    pub fn new(
        sender: impl Into<String>,
        denom: impl Into<String>,
        new_admin: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            denom: denom.into(),
            new_admin: new_admin.into(),
        }
    }
}

impl StargateMsg for MsgChangeAdmin {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgChangeAdmin"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_string(2, &self.denom)
            .append_string(3, &self.new_admin)
            .into_vec()
    }
}

pub struct MsgChangeAdminResponse;

pub struct MsgSetBeforeSendHook {
    pub sender: String,
    pub denom: String,
    pub cosmwasm_address: String,
}

impl MsgSetBeforeSendHook {
    pub fn new(
        sender: impl Into<String>,
        denom: impl Into<String>,
        cosmwasm_address: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            denom: denom.into(),
            cosmwasm_address: cosmwasm_address.into(),
        }
    }
}

impl StargateMsg for MsgSetBeforeSendHook {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgSetBeforeSendHook"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_string(2, &self.denom)
            .append_string(3, &self.cosmwasm_address)
            .into_vec()
    }
}

pub struct MsgSetBeforeSendHookResponse;

pub struct MsgSetDenomMetadata {
    pub sender: String,
    pub metadata: Metadata,
}

impl MsgSetDenomMetadata {
    pub fn new(sender: impl Into<String>, metadata: Metadata) -> Self {
        Self {
            sender: sender.into(),
            metadata,
        }
    }
}

impl StargateMsg for MsgSetDenomMetadata {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_bytes(2, self.metadata.to_buf())
            .into_vec()
    }
}

pub struct MsgSetDenomMetadataResponse;

pub struct MsgForceTransfer {
    pub sender: String,
    pub amount: Coin,
    pub transfer_from_address: String,
    pub transfer_to_address: String,
}

impl MsgForceTransfer {
    pub fn new(
        sender: impl Into<String>,
        amount: Coin,
        transfer_from_address: impl Into<String>,
        transfer_to_address: impl Into<String>,
    ) -> Self {
        Self {
            sender: sender.into(),
            amount,
            transfer_from_address: transfer_from_address.into(),
            transfer_to_address: transfer_to_address.into(),
        }
    }
}

impl StargateMsg for MsgForceTransfer {
    fn url() -> &'static str {
        "/osmosis.tokenfactory.v1beta1.MsgForceTransfer"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.sender)
            .append_bytes(2, self.amount.to_buf())
            .append_string(3, &self.transfer_from_address)
            .append_string(4, &self.transfer_to_address)
            .into_vec()
    }
}

pub struct MsgForceTransferResponse;
