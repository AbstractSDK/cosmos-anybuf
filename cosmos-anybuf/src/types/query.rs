use anybuf::{Anybuf, Bufany};

// PageRequest is to be embedded in gRPC request messages for efficient
// pagination. Ex:
//
//  message SomeRequest {
//          Foo some_parameter = 1;
//          PageRequest pagination = 2;
//  }
pub struct PageRequest {
    // key is a value returned in PageResponse.next_key to begin
    // querying the next page most efficiently. Only one of offset or key
    // should be set.
    pub key: Vec<u8>, // 1

    // offset is a numeric offset that can be used when key is unavailable.
    // It is less efficient than using key. Only one of offset or key should
    // be set.
    pub offset: u64, // 2

    // limit is the total number of results to be returned in the result page.
    // If left empty it will default to a value to be set by each app.
    pub limit: u64, // 3

    // count_total is set to true  to indicate that the result set should include
    // a count of the total number of items available for pagination in UIs.
    // count_total is only respected when offset is used. It is ignored when key
    // is set.
    pub count_total: bool, // 4

    // reverse is set to true if results are to be returned in the descending order.
    //
    // Since: cosmos-sdk 0.43
    pub reverse: bool, // 5
}

impl PageRequest {
    pub fn new(key: Vec<u8>, offset: u64, limit: u64, count_total: bool, reverse: bool) -> Self {
        Self {
            key,
            offset,
            limit,
            count_total,
            reverse,
        }
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_bytes(1, &self.key)
            .append_uint64(2, self.offset)
            .append_uint64(3, self.limit)
            .append_bool(4, self.count_total)
            .append_bool(5, self.reverse)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }
}

// PageResponse is to be embedded in gRPC response messages where the
// corresponding request message has used PageRequest.
//
//  message SomeResponse {
//          repeated Bar results = 1;
//          PageResponse page = 2;
//  }
pub struct PageResponse {
    // next_key is the key to be passed to PageRequest.key to
    // query the next page most efficiently. It will be empty if
    // there are no more results.
    pub next_key: Vec<u8>, // 1

    // total is total number of results available if PageRequest.count_total
    // was set, its value is undefined otherwise
    pub total: u64, // 2
}

impl PageResponse {
    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let next_key = bufany.bytes(1)?;
        let total = bufany.uint64(2)?;
        Some(Self { next_key, total })
    }
}
