use anybuf::Anybuf;

/// Describes a KV key for which you want to get value from the storage on remote chain
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
}
