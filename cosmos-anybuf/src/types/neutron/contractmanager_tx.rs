use crate::types::neutron::contractmanager::Params;
use crate::StargateMsg;
use anybuf::Anybuf;

pub struct MsgUpdateParams {
    pub authority: String,
    pub params: Params,
}

impl MsgUpdateParams {
    pub fn new(authority: impl Into<String>, params: Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }
}

impl StargateMsg for MsgUpdateParams {
    fn url() -> &'static str {
        "/neutron.contractmanager.MsgUpdateParams"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.authority)
            .append_bytes(2, self.params.to_buf())
            .into_vec()
    }
}
