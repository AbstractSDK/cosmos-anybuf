use anybuf::{Anybuf, Bufany};

use crate::{
    types::coin::{to_repeated_message, Coin},
    StargateResponse,
};

pub struct Fee {
    pub recv_fee: Vec<Coin>,    // 1
    pub ack_fee: Vec<Coin>,     // 2
    pub timeout_fee: Vec<Coin>, // 3
}

impl Fee {
    pub fn to_anybuf(&self) -> Anybuf {
        let recv_anybuf: Vec<Anybuf> = to_repeated_message(&self.recv_fee);
        let ack_anybuf: Vec<Anybuf> = to_repeated_message(&self.ack_fee);
        let timeout_anybuf: Vec<Anybuf> = to_repeated_message(&self.timeout_fee);
        Anybuf::new()
            .append_repeated_message(1, &recv_anybuf)
            .append_repeated_message(2, &ack_anybuf)
            .append_repeated_message(3, &timeout_anybuf)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn from_bufany(buf: Bufany) -> Option<Self> {
        let recv_fee = Coin::from_repeated_bytes(buf.repeated_bytes(1)?)?;
        let ack_fee = Coin::from_repeated_bytes(buf.repeated_bytes(2)?)?;
        let timeout_fee = Coin::from_repeated_bytes(buf.repeated_bytes(3)?)?;
        Some(Self {
            recv_fee,
            ack_fee,
            timeout_fee,
        })
    }
}

pub struct Params {
    pub min_fee: Fee, // 1
}

impl Params {
    pub fn from_bufany(buf: Bufany) -> Option<Self> {
        let min_fee_message = buf.message(1)?;
        let min_fee = Fee::from_bufany(min_fee_message)?;
        Some(Self { min_fee })
    }
}

impl StargateResponse for Params {
    fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }
}
