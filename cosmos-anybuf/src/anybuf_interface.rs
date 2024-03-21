use cosmwasm_std::{CosmosMsg, Empty, QueryRequest};

pub trait StargateMsg: Sized {
    /// URL for the gRPC endpoint.
    fn url() -> &'static str;
    /// Serialize to protobuf bytes.
    fn to_buf(&self) -> Vec<u8>;
    /// Create [CosmosMsg] from type.
    fn to_msg(self) -> CosmosMsg {
        CosmosMsg::Stargate {
            type_url: Self::url().to_string(),
            value: cosmwasm_std::Binary(self.to_buf()),
        }
    }
}

pub trait StargateQuery: Sized {
    /// URL for the gRPC endpoint.
    fn url() -> &'static str;
    /// Serialize to protobuf bytes.
    fn to_buf(&self) -> Vec<u8>;
    /// Create [QueryRequest] from type.
    fn to_query(self) -> QueryRequest<Empty> {
        QueryRequest::Stargate {
            path: Self::url().to_string(),
            data: cosmwasm_std::Binary(self.to_buf()),
        }
    }
}

pub trait StargateResponse: Sized {
    /// Deserialize from protobuf bytes.
    fn from_buf(buf: Vec<u8>) -> Option<Self>;
}
