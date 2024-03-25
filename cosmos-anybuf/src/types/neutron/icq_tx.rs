use crate::{StargateMsg, StargateResponse};
use anybuf::{Anybuf, Bufany};

use super::interchainqueries::{KVKey, Params};

pub struct MsgRegisterInterchainQuery {
    pub query_type: String,          // 1
    pub keys: Vec<KVKey>,            // 2
    pub transactions_filter: String, // 3
    pub connection_id: String,       // 4
    pub update_period: u64,          // 5
    pub sender: String,              // 6
}

impl StargateMsg for MsgRegisterInterchainQuery {
    fn url() -> &'static str {
        "/neutron.interchainqueries.MsgRegisterInterchainQuery"
    }

    fn to_buf(&self) -> Vec<u8> {
        let keys: Vec<Anybuf> = self.keys.iter().map(KVKey::to_anybuf).collect();

        Anybuf::new()
            .append_string(1, &self.query_type)
            .append_repeated_message(2, &keys)
            .append_string(3, &self.transactions_filter)
            .append_string(4, &self.connection_id)
            .append_uint64(5, self.update_period)
            .append_string(6, &self.sender)
            .into_vec()
    }
}

pub struct MsgRegisterInterchainQueryResponse {
    pub id: u64, // 1
}

impl StargateResponse for MsgRegisterInterchainQueryResponse {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let buf = Bufany::deserialize(&buf).ok()?;
        let id = buf.uint64(1)?;
        Some(Self { id })
    }
}

pub struct MsgRemoveInterchainQueryRequest {
    pub query_id: u64,  // 1
    pub sender: String, // 2
}

impl StargateMsg for MsgRemoveInterchainQueryRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.MsgRemoveInterchainQueryRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_uint64(1, self.query_id)
            .append_string(2, &self.sender)
            .into_vec()
    }
}

pub struct MsgUpdateInterchainQueryRequest {
    pub query_id: u64,                   // 1
    pub new_keys: Vec<KVKey>,            // 2
    pub new_update_period: u64,          // 3
    pub new_transactions_filter: String, // 4
    pub sender: String,                  // 5
}

impl StargateMsg for MsgUpdateInterchainQueryRequest {
    fn url() -> &'static str {
        "/neutron.interchainqueries.MsgUpdateInterchainQueryRequest"
    }

    fn to_buf(&self) -> Vec<u8> {
        let keys: Vec<Anybuf> = self.new_keys.iter().map(KVKey::to_anybuf).collect();
        Anybuf::new()
            .append_uint64(1, self.query_id)
            .append_repeated_message(2, &keys)
            .append_uint64(3, self.new_update_period)
            .append_string(4, &self.new_transactions_filter)
            .append_string(5, &self.sender)
            .into_vec()
    }
}

pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    pub authority: String, // 1
    /// params defines the x/interchainqueries parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    pub params: Params, // 2
}

impl StargateMsg for MsgUpdateParams {
    fn url() -> &'static str {
        "/neutron.interchainqueries.MsgUpdateParams"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.authority)
            .append_message(2, &self.params.to_anybuf())
            .into_vec()
    }
}
