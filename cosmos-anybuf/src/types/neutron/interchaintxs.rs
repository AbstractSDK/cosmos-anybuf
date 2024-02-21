use crate::types::coin::Coin;
use anybuf::{Anybuf, Bufany};

pub struct Params {
    pub msg_submit_tx_max_messages: u64, // 1
    pub register_fee: Vec<Coin>,         // 2
}

impl Params {
    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let msg_submit_tx_max_messages = bufany.uint64(1)?;
        let coins_buf = bufany.repeated_bytes(2)?;
        let register_fee = Coin::from_repeated_bytes(coins_buf)?;
        Some(Self {
            msg_submit_tx_max_messages,
            register_fee,
        })
    }

    pub fn to_anybuf(&self) -> Anybuf {
        let coins_anybuf: Vec<Anybuf> = self
            .register_fee
            .iter()
            .map(Coin::to_anybuf)
            .collect::<Vec<_>>();
        Anybuf::new()
            .append_uint64(1, self.msg_submit_tx_max_messages)
            .append_repeated_message(2, &coins_anybuf)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }
}
