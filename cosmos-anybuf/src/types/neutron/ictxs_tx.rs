use crate::{
    types::{coin::Coin, neutron::feerefunder::Fee, neutron::interchaintxs::Params},
    StargateMsg,
};
use anybuf::Anybuf;

pub struct MsgRegisterInterchainAccount {
    pub from_address: String,
    pub connection_id: String,
    pub interchain_account_id: String,
    pub register_fee: Vec<Coin>,
}

impl MsgRegisterInterchainAccount {
    pub fn new(
        from_address: impl Into<String>,
        connection_id: impl Into<String>,
        interchain_account_id: impl Into<String>,
        register_fee: Vec<Coin>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            connection_id: connection_id.into(),
            interchain_account_id: interchain_account_id.into(),
            register_fee,
        }
    }
}

impl StargateMsg for MsgRegisterInterchainAccount {
    fn url() -> &'static str {
        "/neutron.interchaintxs.v1.MsgRegisterInterchainAccount"
    }

    fn to_buf(&self) -> Vec<u8> {
        let mut buf = Anybuf::new()
            .append_string(1, &self.from_address)
            .append_string(2, &self.connection_id)
            .append_string(3, &self.interchain_account_id);
        for coin in &self.register_fee {
            buf = buf.append_bytes(4, &coin.to_buf());
        }
        buf.into_vec()
    }
}

pub struct MsgSubmitTx {
    pub from_address: String,
    pub interchain_account_id: String,
    pub connection_id: String,
    pub msgs: Vec<Anybuf>, // Assuming Anybuf is used for google.protobuf.Any
    pub memo: String,
    pub timeout: u64,
    pub fee: Fee,
}

impl MsgSubmitTx {
    pub fn new(
        from_address: impl Into<String>,
        interchain_account_id: impl Into<String>,
        connection_id: impl Into<String>,
        msgs: Vec<Anybuf>,
        memo: impl Into<String>,
        timeout: u64,
        fee: Fee,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            interchain_account_id: interchain_account_id.into(),
            connection_id: connection_id.into(),
            msgs,
            memo: memo.into(),
            timeout,
            fee,
        }
    }
}

impl StargateMsg for MsgSubmitTx {
    fn url() -> &'static str {
        "/neutron.interchaintxs.v1.MsgSubmitTx"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.from_address)
            .append_string(2, &self.interchain_account_id)
            .append_string(3, &self.connection_id)
            .append_repeated_message(4, &self.msgs)
            .append_string(5, &self.memo)
            .append_uint64(6, self.timeout)
            .append_bytes(7, self.fee.to_buf())
            .into_vec()
    }
}

pub struct MsgUpdateParams {
    pub authority: String,
    pub params: Params,
}

impl MsgUpdateParams {
    pub fn new(authority: impl Into<String>, params: Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }
}

impl StargateMsg for MsgUpdateParams {
    fn url() -> &'static str {
        "/neutron.interchaintxs.v1.MsgUpdateParams"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.authority)
            .append_bytes(2, self.params.to_buf())
            .into_vec()
    }
}
