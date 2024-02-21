use std::fs;

use super::ProtoModule;

#[derive(Clone, Debug)]
pub struct ProtoTx(pub String);

impl AsRef<str> for ProtoTx {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ProtoModule for ProtoTx {
    /// Returns an example of how a proto file should be encoded
    fn example() -> (String, String) {
        let tx_proto = fs::read_to_string("example/tokenfactory_tx.proto").unwrap();
        let tx_rust = fs::read_to_string("example/tokenfactory_tx.rs").unwrap();
        (tx_proto, tx_rust)
    }

    fn postfix() -> &'static str {
        "tx"
    }
}

impl ProtoTx {
    pub fn new(tx_proto_url: impl Into<String>) -> Self {
        Self(tx_proto_url.into())
    }
}

impl From<&str> for ProtoTx {
    fn from(tx_proto_url: &str) -> Self {
        Self::new(tx_proto_url)
    }
}
