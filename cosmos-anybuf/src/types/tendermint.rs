use anybuf::Anybuf;

// Crypto

pub struct ProofOp {
    pub ty: String,    // 1
    pub key: Vec<u8>,  // 2
    pub data: Vec<u8>, // 3
}

impl ProofOp {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_string(1, &self.ty)
            .append_bytes(2, &self.key)
            .append_bytes(3, &self.data)
    }
}

pub struct ProofOps {
    pub ops: Vec<ProofOp>, // 1
}

impl ProofOps {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        let ops: Vec<Anybuf> = self.ops.iter().map(ProofOp::to_anybuf).collect();
        Anybuf::new().append_repeated_message(1, &ops)
    }
}

pub struct Proof {
    pub total: i64,          // 1
    pub index: i64,          // 2
    pub leaf_hash: Vec<u8>,  // 3
    pub aunts: Vec<Vec<u8>>, // 4
}

impl Proof {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_int64(1, self.total)
            .append_int64(2, self.index)
            .append_bytes(3, &self.leaf_hash)
            .append_repeated_bytes(4, &self.aunts)
    }
}

// Abci

pub struct ResponseDeliverTx {
    pub code: u32,          // 1
    pub data: Vec<u8>,      // 2
    pub log: String,        // 3
    pub info: String,       // 4
    pub gas_wanted: i64,    // 5
    pub gas_used: i64,      // 6
    pub events: Vec<Event>, // 7
    pub codespace: String,  // 8
}

impl ResponseDeliverTx {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        let events: Vec<Anybuf> = self.events.iter().map(Event::to_anybuf).collect();
        Anybuf::new()
            .append_uint32(1, self.code)
            .append_bytes(2, &self.data)
            .append_string(3, &self.log)
            .append_string(4, &self.info)
            .append_int64(5, self.gas_wanted)
            .append_int64(6, self.gas_used)
            .append_repeated_message(7, &events)
            .append_string(8, &self.codespace)
    }
}

pub struct Event {
    pub ty: String,                      // 1
    pub attributes: Vec<EventAttribute>, // 2
}

impl Event {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        let attributes: Vec<Anybuf> = self
            .attributes
            .iter()
            .map(EventAttribute::to_anybuf)
            .collect();
        Anybuf::new()
            .append_string(1, &self.ty)
            .append_repeated_message(2, &attributes)
    }
}

pub struct EventAttribute {
    pub key: String,   // 1
    pub value: String, //  2
    pub index: bool,   //  3
}

impl EventAttribute {
    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_string(1, &self.key)
            .append_string(2, &self.value)
            .append_bool(3, self.index)
    }
}
