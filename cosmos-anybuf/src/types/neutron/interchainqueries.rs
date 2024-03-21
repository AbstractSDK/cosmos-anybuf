use anybuf::{Anybuf, Bufany};

use crate::types::coin::Coin;

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
