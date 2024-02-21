// manually implemented types so chatgpt don't jump through files
use anybuf::{Anybuf, Bufany};

pub struct Params {
    pub sudo_call_gas_limit: u64, // 1
}

impl Params {
    pub fn new(sudo_call_gas_limit: u64) -> Self {
        Self {
            sudo_call_gas_limit,
        }
    }

    pub fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_uint64(1, self.sudo_call_gas_limit)
            .into_vec()
    }

    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        Some(Self {
            sudo_call_gas_limit: bufany.uint64(1)?,
        })
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }
}

pub struct Failure {
    pub address: String,       // 1
    pub id: u64,               // 2
    pub sudo_payload: Vec<u8>, // 3
    pub error: String,         // 4
}

impl Failure {
    pub fn new<S: Into<String>>(address: S, id: u64, sudo_payload: Vec<u8>, error: S) -> Self {
        Self {
            address: address.into(),
            id,
            sudo_payload,
            error: error.into(),
        }
    }

    pub fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.address)
            .append_uint64(2, self.id)
            .append_bytes(3, &self.sudo_payload)
            .append_string(4, &self.error)
            .into_vec()
    }

    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let address = bufany.string(1)?;
        let id = bufany.uint64(2)?;
        let sudo_payload = bufany.bytes(1)?;
        let error = bufany.string(4)?;

        Some(Self {
            address,
            id,
            sudo_payload,
            error,
        })
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }
}
