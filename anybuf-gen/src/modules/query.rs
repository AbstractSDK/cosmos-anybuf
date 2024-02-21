use std::fs;

use super::ProtoModule;

#[derive(Clone, Debug)]
pub struct ProtoQuery(pub String);

impl AsRef<str> for ProtoQuery {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ProtoModule for ProtoQuery {
    fn example() -> (String, String) {
        let tx_proto = fs::read_to_string("example/tokenfactory_query.proto").unwrap();
        let tx_rust = fs::read_to_string("example/tokenfactory_query.rs").unwrap();
        (tx_proto, tx_rust)
    }

    fn postfix() -> &'static str {
        "query"
    }
}

impl ProtoQuery {
    pub fn new(query_proto_url: impl Into<String>) -> Self {
        Self(query_proto_url.into())
    }
}

impl From<&str> for ProtoQuery {
    fn from(query_proto_url: &str) -> Self {
        Self::new(query_proto_url)
    }
}
