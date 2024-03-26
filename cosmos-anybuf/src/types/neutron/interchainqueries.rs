use anybuf::{Anybuf, Bufany};

use crate::types::{
    coin::{self, Coin},
    ibc::Height,
};

pub struct Params {
    pub query_submit_timeout: u64,   // 1
    pub query_deposit: Vec<Coin>,    // 2
    pub tx_query_removal_limit: u64, // 3
}

impl Params {
    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let query_submit_timeout = bufany.uint64(1)?;
        let coins_buf = bufany.repeated_bytes(2)?;
        let query_deposit = Coin::from_repeated_bytes(coins_buf)?;
        let tx_query_removal_limit = bufany.uint64(3)?;
        Some(Self {
            query_submit_timeout,
            query_deposit,
            tx_query_removal_limit,
        })
    }

    pub fn to_anybuf(&self) -> Anybuf {
        let coins_anybuf: Vec<Anybuf> = self.query_deposit.iter().map(Coin::to_anybuf).collect();
        Anybuf::new()
            .append_uint64(1, self.query_submit_timeout)
            .append_repeated_message(2, &coins_anybuf)
            .append_uint64(3, self.tx_query_removal_limit)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }
}

/// Describes a KV key for which you want to get value from the storage on remote chain
#[cosmwasm_schema::cw_serde]
pub struct KVKey {
    /// Path (storage prefix) to the storage where you want to read value by key
    /// (usually name of cosmos-sdk module: 'staking', 'bank', etc.)
    pub path: String,

    /// Key you want to read from the storage
    pub key: Vec<u8>,
}

impl KVKey {
    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_string(1, &self.path)
            .append_bytes(2, &self.key)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let path = deserialized.string(1)?;
        let key = deserialized.bytes(2)?;
        Some(Self { path, key })
    }
}

#[cosmwasm_schema::cw_serde]
pub struct RegisteredQuery {
    // The unique id of the registered query.
    pub id: u64, // 1

    // The address that registered the query.
    pub owner: String, // 2

    // The query type identifier: `kv` or `tx` now
    pub query_type: String, // 3

    // The KV-storage keys for which we want to get values from remote chain
    pub keys: Vec<KVKey>, // 4

    // The filter for transaction search ICQ
    pub transactions_filter: String, // 5

    // The IBC connection ID for getting ConsensusState to verify proofs
    pub connection_id: String, // 6

    // Parameter that defines how often the query must be updated.
    pub update_period: u64, // 7

    // The local chain last block height when the query result was updated.
    pub last_submitted_result_local_height: u64, // 8

    // The remote chain last block height when the query result was updated.
    pub last_submitted_result_remote_height: Height, // 9

    // Amount of coins deposited for the query.
    pub deposit: Vec<Coin>, // 10

    // Timeout before query becomes available for everybody to remove.
    pub submit_timeout: u64, // 11

    // The local chain height when the query was registered.
    pub registered_at_height: u64, // 12
}

impl RegisteredQuery {
    pub fn to_anybuf(&self) -> Anybuf {
        let keys: Vec<Anybuf> = self.keys.iter().map(KVKey::to_anybuf).collect();
        let deposit = coin::to_repeated_message(&self.deposit);
        Anybuf::new()
            .append_uint64(1, self.id)
            .append_string(2, &self.owner)
            .append_string(3, &self.query_type)
            .append_repeated_message(4, &keys)
            .append_string(5, &self.transactions_filter)
            .append_string(6, &self.connection_id)
            .append_uint64(7, self.update_period)
            .append_uint64(8, self.last_submitted_result_local_height)
            .append_message(9, &self.last_submitted_result_remote_height.to_anybuf())
            .append_repeated_message(10, &deposit)
            .append_uint64(11, self.submit_timeout)
            .append_uint64(12, self.registered_at_height)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }

    pub fn from_bufany(buf: Bufany) -> Option<Self> {
        let id = buf.uint64(1)?;
        let owner = buf.string(2)?;
        let query_type = buf.string(3)?;
        let keys_bytes = buf.repeated_bytes(4)?;
        let keys = keys_bytes
            .into_iter()
            .map(KVKey::from_buf)
            .collect::<Option<Vec<KVKey>>>()?;
        let transactions_filter = buf.string(5)?;
        let connection_id = buf.string(6)?;
        let update_period = buf.uint64(7)?;
        let last_submitted_result_local_height = buf.uint64(8)?;
        let last_submitted_result_remote_height_message = buf.message(9)?;
        let last_submitted_result_remote_height =
            Height::from_bufany(last_submitted_result_remote_height_message)?;
        let deposit_bytes = buf.repeated_bytes(10)?;
        let deposit = coin::Coin::from_repeated_bytes(deposit_bytes)?;
        let submit_timeout = buf.uint64(11)?;
        let registered_at_height = buf.uint64(12)?;

        Some(Self {
            id,
            owner,
            query_type,
            keys,
            transactions_filter,
            connection_id,
            update_period,
            last_submitted_result_local_height,
            last_submitted_result_remote_height,
            deposit,
            submit_timeout,
            registered_at_height,
        })
    }
}

pub struct QueryResult {
    pub kv_results: Vec<StorageValue>, // 1
    pub height: u64,                   // 3
    pub revision: u64,                 // 4
                                       // TODO: don't think contract should care about those fields
                                       // pub block: Block,             // 2
                                       // pub allow_kv_callbacks: bool, // 5
}

impl QueryResult {
    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let buf = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(buf)
    }

    pub fn from_bufany(buf: Bufany) -> Option<Self> {
        let kv_results_bytes = buf.repeated_bytes(1)?;
        let kv_results = kv_results_bytes
            .into_iter()
            .map(StorageValue::from_buf)
            .collect::<Option<_>>()?;
        let height = buf.uint64(3)?;
        let revision = buf.uint64(4)?;
        Some(Self {
            kv_results,
            height,
            revision,
            // allow_kv_callbacks: todo!(),
        })
    }
}

pub struct StorageValue {
    // is the substore name (acc, staking, etc.)
    pub storage_prefix: String, //  1

    // is the key in IAVL store
    pub key: Vec<u8>, // 2

    // is the value in IAVL store
    pub value: Vec<u8>, // 3

                        // is the Merkle Proof which proves existence of key-value pair in IAVL
                        // storage
                        // TODO: don't think contract should care
                        // pub proof: ProofOps, // 4
}

impl StorageValue {
    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let buf = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(buf)
    }

    pub fn from_bufany(buf: Bufany) -> Option<Self> {
        let storage_prefix = buf.string(1)?;
        let key = buf.bytes(2)?;
        let value = buf.bytes(3)?;
        Some(Self {
            storage_prefix,
            key,
            value,
        })
    }
}

// pub struct Block {
//     // We need to know block X+1 to verify response of transaction for block X
//     // since LastResultsHash is root hash of all results from the txs from the
//     // previous block
//     pub next_block_header: Any, // 1

//     // We need to know block X to verify inclusion of transaction for block X
//     pub header: Any, // 2

//     pub tx: TxValue, //  3
// }

// pub struct TxValue {
//     pub response: ResponseDeliverTx, // 1

//     // is the Merkle Proof which proves existence of response in block with height
//     // next_block_header.Height
//     pub delivery_proof: Proof, // 2

//     // is the Merkle Proof which proves existence of data in block with height
//     // header.Height
//     pub inclusion_proof: Proof, // 3

//     // is body of the transaction
//     pub data: Vec<u8>, // 4
// }
