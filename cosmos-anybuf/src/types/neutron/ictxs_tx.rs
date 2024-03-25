use crate::{
    types::{
        coin::Coin,
        neutron::{feerefunder::Fee, interchaintxs::Params},
    },
    StargateMsg, StargateResponse,
};
use anybuf::{Anybuf, Bufany};

pub struct MsgRegisterInterchainAccount {
    pub from_address: String,          // 1
    pub connection_id: String,         // 2
    pub interchain_account_id: String, // 3
    pub register_fee: Vec<Coin>,       // 4
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
    pub from_address: String,          // 1
    pub interchain_account_id: String, //2
    pub connection_id: String,         // 3
    pub msgs: Vec<Anybuf>,             // 4
    pub memo: String,                  // 5
    pub timeout: u64,                  // 6
    pub fee: Fee,                      // 7
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

pub struct MsgSubmitTxResponse {
    pub sequence_id: u64, // 1
    pub channel: String,  // 2
}

impl StargateResponse for MsgSubmitTxResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let sequence_id = deserialized.uint64(1)?;
        let channel = deserialized.string(2)?;

        Some(Self {
            sequence_id,
            channel,
        })
    }
}

pub struct MsgUpdateParams {
    pub authority: String,
    pub params: Params,
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
