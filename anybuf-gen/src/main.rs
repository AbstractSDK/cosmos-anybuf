pub mod chains;

use anybuf_gen::modules::Module;

use anyhow::Result as AResult;

use crate::chains::{Protos, TOKENFACTORY};

lazy_static::lazy_static! {
    static ref PROTOS: Vec<chains::Protos> = {
        vec![
            Protos {
                chain_name: "osmosis".to_string(),
                whitelist_url: "https://github.com/osmosis-labs/osmosis/blob/main/wasmbinding/stargate_whitelist.go".to_string(),
                modules: vec![
                    Module {
                        name: TOKENFACTORY.to_string(),
                        // tx_url: Some("https://github.com/osmosis-labs/osmosis/blob/main/proto/osmosis/tokenfactory/v1beta1/tx.proto".into()),
                        query_url: Some("https://github.com/osmosis-labs/osmosis/blob/main/proto/osmosis/tokenfactory/v1beta1/query.proto".into()),
                        ..Default::default()
                    }
                ]
            },
            Protos {
                chain_name: "neutron".to_string(),
                whitelist_url: "https://github.com/neutron-org/neutron/blob/main/wasmbinding/stargate_allowlist.go".to_owned(),
                modules: vec![
                    Module {
                        name: TOKENFACTORY.to_string(),
                        // tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/osmosis/tokenfactory/v1beta1/tx.proto".into()),
                        // query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/osmosis/tokenfactory/v1beta1/query.proto".into()),
                        .. Default::default()
                    },
                    Module {
                        name: "icq".to_string(),
                        // TODO: chatgpt not capable of generating
                        // tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/interchainqueries/tx.proto".into()),
                        // query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/interchainqueries/query.proto".into()),
                        .. Default::default()
                    },
                    Module {
                        name: "contractmanager".to_string(),
                        // tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/contractmanager/tx.proto".into()),
                        // query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/contractmanager/query.proto".into()),
                        .. Default::default()
                    },
                    // TODO: chatgpt not capable of generating
                    Module {
                        name: "dex".to_string(),
                        // tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/dex/tx.proto".into()),
                        // query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/dex/query.proto".into()),
                        .. Default::default()
                    },
                    // This module has queries of amounts burned neutrons in a block
                    // Module {
                    //     name: "feeburner".to_string(),
                    //     tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/feeburner/tx.proto".into()),
                    //     query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/feeburner/query.proto".into()),
                    //       ..Default::default()
                    // },
                    // This module is used for querying ibc refund fees
                    // TODO: Is it useful for debug anything?
                    // Module {
                    //     name: "feerefunder".to_string(),
                    //     tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/feerefunder/tx.proto".into()),
                    //     query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/feerefunder/query.proto".into()),
                    //     ..Default::default()
                    // },
                    Module {
                        name: "ictxs".to_string(),
                        // tx_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/interchaintxs/v1/tx.proto".into()),
                        // query_url: Some("https://github.com/neutron-org/neutron/blob/main/proto/neutron/interchaintxs/v1/query.proto".into()),
                        ..Default::default()
                    }
                ]
            }
        ]
    };
}

#[tokio::main]
async fn main() -> AResult<()> {
    dotenv::dotenv().unwrap();

    for proto in PROTOS.iter() {
        for module in &proto.modules {
            module
                .process_module(&proto.chain_name, &proto.whitelist_url)
                .await?;
        }
    }
    Ok(())
}
