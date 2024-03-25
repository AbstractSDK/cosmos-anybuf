//! Copied from https://github.com/neutron-org/neutron-sdk/blob/612ea5ac87d5760d2a4f6311ab5bdabd0bbbe5b4/packages/neutron-sdk/src/sudo/msg.rs

use cosmwasm_std::Binary;

#[cosmwasm_schema::cw_serde]
pub struct Height {
    /// the revision that the client is currently on
    #[serde(default)]
    pub revision_number: u64,
    /// **height** is a height of remote chain
    #[serde(default)]
    pub revision_height: u64,
}

#[cosmwasm_schema::cw_serde]
pub struct RequestPacket {
    pub sequence: Option<u64>,
    pub source_port: Option<String>,
    pub source_channel: Option<String>,
    pub destination_port: Option<String>,
    pub destination_channel: Option<String>,
    pub data: Option<Binary>,
    pub timeout_height: Option<RequestPacketTimeoutHeight>,
    pub timeout_timestamp: Option<u64>,
}
#[cosmwasm_schema::cw_serde]
pub struct RequestPacketTimeoutHeight {
    pub revision_number: Option<u64>,
    pub revision_height: Option<u64>,
}

#[cosmwasm_schema::cw_serde]
pub enum SudoMsg {
    Response {
        request: RequestPacket,
        data: Binary,
    },
    Error {
        request: RequestPacket,
        details: String,
    },
    Timeout {
        request: RequestPacket,
    },
    OpenAck {
        port_id: String,
        channel_id: String,
        counterparty_channel_id: String,
        counterparty_version: String,
    },
    TxQueryResult {
        query_id: u64,
        height: Height,
        data: Binary,
    },
    #[serde(rename = "kv_query_result")]
    KVQueryResult {
        query_id: u64,
    },
}

/// TransferSudoMsg is a sudo response payload for a native ibc transfer
/// SudoMsg for ibc transfer has fewer methods than SudoMsg for ica txs
/// so we describe standalone type to not confuse users with useless variants
#[cosmwasm_schema::cw_serde]
pub enum TransferSudoMsg {
    Response {
        request: RequestPacket,
        data: Binary,
    },
    Error {
        request: RequestPacket,
        details: String,
    },
    Timeout {
        request: RequestPacket,
    },
}
