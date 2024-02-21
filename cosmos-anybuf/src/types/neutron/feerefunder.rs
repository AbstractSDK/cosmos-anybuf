use anybuf::Anybuf;

use crate::types::coin::{to_repeated_message, Coin};

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
}
