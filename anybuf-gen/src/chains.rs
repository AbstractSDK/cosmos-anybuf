use anybuf_gen::modules::Module;

pub const TOKENFACTORY: &'static str = "tokenfactory";

#[derive(Debug, Clone)]
pub struct Protos {
    pub chain_name: String,
    pub whitelist_url: String,
    pub modules: Vec<Module>,
}
