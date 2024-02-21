use anybuf::{Anybuf, Bufany};

// CosmosSDK Coin: https://github.com/cosmos/cosmos-sdk/blob/main/proto/cosmos/base/v1beta1/coin.proto
#[cosmwasm_schema::cw_serde]
pub struct Coin {
    pub denom: String,  // 1
    pub amount: String, // 2
}

impl Coin {
    pub fn new(amount: impl Into<u128>, denom: impl Into<String>) -> Self {
        Self {
            amount: amount.into().to_string(),
            denom: denom.into(),
        }
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_string(1, &self.denom)
            .append_string(2, &self.amount)
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let denom = deserialized.string(1)?;
        let amount = deserialized.string(2)?.parse().ok()?;
        Some(Self { amount, denom })
    }

    pub fn from_repeated_bytes(repeated_bytes: Vec<Vec<u8>>) -> Option<Vec<Self>> {
        repeated_bytes.into_iter().map(Self::from_buf).collect()
    }
}

pub fn to_repeated_message(coins: &[Coin]) -> Vec<Anybuf> {
    coins.iter().map(Coin::to_anybuf).collect::<Vec<_>>()
}

impl From<cosmwasm_std::Coin> for Coin {
    fn from(value: cosmwasm_std::Coin) -> Self {
        Self::new(value.amount, value.denom)
    }
}
