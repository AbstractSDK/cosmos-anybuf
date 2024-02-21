use anybuf::{Anybuf, Bufany};

// https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/timestamp.proto
pub struct Timestamp {
    // Represents seconds of UTC time since Unix epoch
    // 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    // 9999-12-31T23:59:59Z inclusive.
    pub seconds: i64, // 1

    // Non-negative fractions of a second at nanosecond resolution. Negative
    // second values with fractions must still have non-negative nanos values
    // that count forward in time. Must be from 0 to 999,999,999
    // inclusive.
    pub nanos: i32, // 2
}

impl Timestamp {
    pub fn new(seconds: i64, nanos: i32) -> Self {
        Self { seconds, nanos }
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_int64(1, self.seconds)
            .append_int32(2, self.nanos)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let deserialized = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(deserialized)
    }

    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let seconds = bufany.int64(1)?;
        let nanos = bufany.int32(2)?;
        Some(Self { seconds, nanos })
    }
}
