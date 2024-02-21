// manually implemented types so chatgpt don't jump through files
use anybuf::{Anybuf, Bufany};

use crate::types::coin::Coin;

pub struct Params {
    pub denom_creation_fee: Vec<Coin>,   // 1
    pub denom_creation_gas_consume: u64, // 2
}

impl Params {
    pub fn new(denom_creation_fee: Vec<Coin>, denom_creation_gas_consume: u64) -> Self {
        Self {
            denom_creation_fee,
            denom_creation_gas_consume,
        }
    }

    pub fn to_buf(&self) -> Vec<u8> {
        let coins_anybuf: Vec<Anybuf> = self
            .denom_creation_fee
            .iter()
            .map(|c| c.to_anybuf())
            .collect::<Vec<_>>();
        Anybuf::new()
            .append_repeated_message(1, &coins_anybuf)
            .append_uint64(2, self.denom_creation_gas_consume)
            .into_vec()
    }

    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let coins_buf = bufany.repeated_bytes(1)?;
        let denom_creation_fee: Vec<Coin> = Coin::from_repeated_bytes(coins_buf)?;
        let denom_creation_gas_consume: u64 = bufany.uint64(2)?;
        Some(Self {
            denom_creation_fee,
            denom_creation_gas_consume,
        })
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }
}

pub struct DenomAuthorityMetadata {
    pub admin: String,
}

impl DenomAuthorityMetadata {
    pub fn new(admin: impl Into<String>) -> Self {
        Self {
            admin: admin.into(),
        }
    }

    pub fn to_buf(&self) -> Vec<u8> {
        Anybuf::new().append_string(1, &self.admin).into_vec()
    }

    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let admin = bufany.string(1)?;
        Some(Self { admin })
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }
}

#[cfg(test)]
mod test {
    use crate::types::coin::Coin;

    // Was wondering if we can deserialize messages from bytes
    // while Bufany::repeated_message implementation missing:
    // https://github.com/noislabs/anybuf/issues/11
    #[test]
    fn coins_from_repeated_bytes_work() {
        let coins = vec![
            Coin::new(42_u128, "aloha".to_string()),
            Coin::new(1337_u128, "l33t".to_string()),
        ];
        let coins_anybuf: Vec<anybuf::Anybuf> =
            coins.iter().map(|c| c.to_anybuf()).collect::<Vec<_>>();

        let serialized = anybuf::Anybuf::new()
            .append_repeated_message(1, &coins_anybuf)
            .into_vec();
        let decoded = anybuf::Bufany::deserialize(&serialized).unwrap();
        let coins_buf = decoded.repeated_bytes(1).unwrap();
        let coins: Vec<Coin> = coins_buf
            .into_iter()
            .map(|coin_buf| Coin::from_buf(coin_buf).unwrap())
            .collect();
        assert_eq!(coins[0].denom, "aloha");
        assert_eq!(coins[0].amount, "42");

        assert_eq!(coins[1].denom, "l33t");
        assert_eq!(coins[1].amount, "1337");
        // assert_eq!(decoded.bytes(4), Some(vec![]));
    }
}
