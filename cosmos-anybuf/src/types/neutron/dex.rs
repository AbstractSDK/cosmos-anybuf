use anybuf::{Anybuf, Bufany};

// Dex Params: https://github.com/neutron-org/neutron/blob/main/proto/neutron/dex/params.proto
#[derive(Clone, Debug)]
pub struct Params {
    pub fee_tiers: Vec<u64>,           // 1
    pub max_true_taker_spread: String, // 2
}

impl Params {
    pub fn new(fee_tiers: Vec<u64>, max_true_taker_spread: impl Into<String>) -> Self {
        Self {
            fee_tiers,
            max_true_taker_spread: max_true_taker_spread.into(),
        }
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_repeated_uint64(1, &self.fee_tiers)
            .append_string(2, &self.max_true_taker_spread)
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        let fee_tiers = deserialized.repeated_uint64(1)?;
        let max_true_taker_spread = deserialized.string(2)?;
        Some(Self {
            fee_tiers,
            max_true_taker_spread,
        })
    }
}
