use anybuf::{Anybuf, Bufany};

#[cosmwasm_schema::cw_serde]
pub struct Height {
    pub revision_number: u64, // 1
    pub revision_height: u64, // 2
}

impl Height {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_uint64(1, self.revision_number)
            .append_uint64(2, self.revision_height)
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }

    pub fn from_bufany(buf: Bufany) -> Option<Self> {
        let revision_number = buf.uint64(1)?;
        let revision_height = buf.uint64(2)?;
        Some(Self {
            revision_number,
            revision_height,
        })
    }
}
