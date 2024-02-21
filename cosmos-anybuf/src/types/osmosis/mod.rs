pub mod tokenfactory_query;
pub mod tokenfactory_tx;

pub(crate) use crate::types::tokenfactory;

// TODO: use Osmosis object
#[cfg(test)]
mod tests {
    use crate::anybuf_interface::*;
    use crate::types::{
        bank::{DenomUnit, Metadata},
        coin::Coin,
    };
    use cosmwasm_std::Binary;
    use osmosis_std::types::cosmos::{
        bank::v1beta1::{DenomUnit as ODenomUnit, Metadata as OMetadata},
        base::v1beta1::Coin as OCoin,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1;

    mod tx {
        use super::*;

        use crate::types::osmosis::tokenfactory_tx;
        use cosmwasm_std::CosmosMsg;

        #[test]
        fn msg_create_denom() {
            let sender = "Bob";
            let subdenom = "Alice";
            let gen_msg = tokenfactory_tx::MsgCreateDenom {
                sender: sender.to_owned(),
                subdenom: subdenom.to_owned(),
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgCreateDenom::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgCreateDenom {
                        sender: sender.to_owned(),
                        subdenom: subdenom.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }

        #[test]
        fn msg_burn() {
            let sender = "Bob";
            let burn_from_address = "Alice";
            let denom = "foo";
            let amount: u128 = 420;
            let gen_msg = tokenfactory_tx::MsgBurn {
                sender: sender.to_owned(),
                amount: Coin::new(amount, denom.to_owned()),
                burn_from_address: burn_from_address.to_owned(),
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgBurn::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgBurn {
                        sender: sender.to_owned(),
                        // TODO: Why it's option XD
                        amount: Some(OCoin {
                            denom: denom.to_owned(),
                            amount: amount.to_string(),
                        }),
                        burn_from_address: burn_from_address.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }

        #[test]
        fn msg_change_admin() {
            let sender = "Bob";
            let denom = "uosmo";
            let new_admin = "Alice";
            let gen_msg = tokenfactory_tx::MsgChangeAdmin {
                sender: sender.to_owned(),
                denom: denom.to_owned(),
                new_admin: new_admin.to_owned(),
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgChangeAdmin::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgChangeAdmin {
                        sender: sender.to_owned(),
                        denom: denom.to_owned(),
                        new_admin: new_admin.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }

        #[test]
        fn msg_force_transfer() {
            let sender = "Bob";
            let denom = "uosmo";
            let amount: u128 = 1337;
            let transfer_from_address = "Foo";
            let transfer_to_address = "Bar";
            let gen_msg = tokenfactory_tx::MsgForceTransfer {
                sender: sender.to_owned(),
                amount: Coin::new(amount, denom.to_owned()),
                transfer_from_address: transfer_from_address.to_owned(),
                transfer_to_address: transfer_to_address.to_owned(),
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgForceTransfer::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgForceTransfer {
                        sender: sender.to_owned(),
                        amount: Some(OCoin {
                            denom: denom.to_owned(),
                            amount: amount.to_string(),
                        }),
                        transfer_from_address: transfer_from_address.to_owned(),
                        transfer_to_address: transfer_to_address.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }

        #[test]
        fn msg_mint() {
            let sender = "Bob";
            let denom = "uosmo";
            let amount: u128 = 1337;
            let mint_to_address = "Alice";
            let gen_msg = tokenfactory_tx::MsgMint {
                sender: sender.to_owned(),
                amount: Coin::new(amount, denom.to_owned()),
                mint_to_address: mint_to_address.to_owned(),
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgMint::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgMint {
                        sender: sender.to_owned(),
                        amount: Some(OCoin {
                            denom: denom.to_owned(),
                            amount: amount.to_string(),
                        }),
                        mint_to_address: mint_to_address.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }

        #[test]
        fn msg_set_before_send_hook() {
            let sender = "Bob";
            let denom = "uosmo";
            let cosmwasm_address = "Alice";
            let gen_msg = tokenfactory_tx::MsgSetBeforeSendHook {
                sender: sender.to_owned(),
                denom: denom.to_string(),
                cosmwasm_address: cosmwasm_address.to_owned(),
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgSetBeforeSendHook::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgSetBeforeSendHook {
                        sender: sender.to_owned(),
                        denom: denom.to_owned(),
                        cosmwasm_address: cosmwasm_address.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }

        #[test]
        fn msg_set_denom_metadata() {
            let sender = "Bob";
            let description = "YEP description";
            let base = "YEP base";
            let display = "YEP display";
            let name = "YEP name";
            let symbol = "YEP symbol";
            let uri = "YEP URI";
            let uri_hash = "YEP URIHASH";

            // DenomUnit
            let denom = "uosmo";
            let exponent = 7;
            let aliases = vec!["yo-osmo".to_owned(), "rofl_osmo".to_owned()];

            let gen_msg = tokenfactory_tx::MsgSetDenomMetadata {
                sender: sender.to_owned(),
                metadata: Metadata {
                    description: description.to_owned(),
                    denom_units: vec![DenomUnit {
                        denom: denom.to_owned(),
                        exponent,
                        aliases: aliases.clone(),
                    }],
                    base: base.to_owned(),
                    display: display.to_owned(),
                    name: name.to_owned(),
                    symbol: symbol.to_owned(),
                    URI: uri.to_owned(),
                    URIHASH: uri_hash.to_owned(),
                },
            }
            .to_msg();

            let expected_msg = CosmosMsg::Stargate {
                type_url: v1beta1::MsgSetDenomMetadata::TYPE_URL.to_owned(),
                value: cosmwasm_std::Binary(
                    v1beta1::MsgSetDenomMetadata {
                        sender: sender.to_owned(),
                        metadata: Some(OMetadata {
                            description: description.to_owned(),
                            denom_units: vec![ODenomUnit {
                                denom: denom.to_owned(),
                                exponent,
                                aliases: aliases.clone(),
                            }],
                            base: base.to_owned(),
                            display: display.to_owned(),
                            name: name.to_owned(),
                            symbol: symbol.to_owned(),
                            // uri: uri.to_owned(),
                            // uri_hash: uri_hash.to_owned(),
                        }),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);
        }
    }

    mod query {
        use super::*;

        use crate::types::osmosis::tokenfactory_query;
        use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
            DenomAuthorityMetadata as ODenomAuthorityMetadata, Params as OParams,
        };

        #[test]
        fn query_params() {
            let gen_msg = tokenfactory_query::QueryParamsRequest {}.to_query();
            let expected_msg = cosmwasm_std::QueryRequest::Stargate {
                path: v1beta1::QueryParamsRequest::TYPE_URL.to_owned(),
                data: Binary(v1beta1::QueryParamsRequest {}.to_proto_bytes()),
            };

            assert_eq!(gen_msg, expected_msg);

            let denom = "uosmo";
            let amount = 42;
            let denom_creation_gas_consume = 567;
            let buf = v1beta1::QueryParamsResponse {
                params: Some(OParams {
                    denom_creation_fee: vec![OCoin {
                        denom: denom.to_owned(),
                        amount: amount.to_string(),
                    }],
                    denom_creation_gas_consume,
                }),
            };
            let parsed =
                tokenfactory_query::QueryParamsResponse::from_buf(buf.to_proto_bytes()).unwrap();
            assert_eq!(
                parsed.params.denom_creation_gas_consume,
                denom_creation_gas_consume
            );
            let coin = parsed.params.denom_creation_fee[0].clone();
            assert_eq!(coin.denom, denom);
            assert_eq!(coin.amount, amount.to_string());
        }

        #[test]
        fn query_denom_authority_metadata() {
            let denom = "uosmo";

            let gen_msg = tokenfactory_query::QueryDenomAuthorityMetadataRequest {
                denom: denom.to_owned(),
            }
            .to_query();
            let expected_msg = cosmwasm_std::QueryRequest::Stargate {
                path: v1beta1::QueryDenomAuthorityMetadataRequest::TYPE_URL.to_owned(),
                data: Binary(
                    v1beta1::QueryDenomAuthorityMetadataRequest {
                        denom: denom.to_owned(),
                    }
                    .to_proto_bytes(),
                ),
            };

            assert_eq!(gen_msg, expected_msg);

            let admin = "Alice";
            let buf = v1beta1::QueryDenomAuthorityMetadataResponse {
                authority_metadata: Some(ODenomAuthorityMetadata {
                    admin: admin.to_owned(),
                }),
            };
            let parsed = tokenfactory_query::QueryDenomAuthorityMetadataResponse::from_buf(
                buf.to_proto_bytes(),
            )
            .unwrap();
            assert_eq!(parsed.authority_metadata.admin, admin);
        }
    }
}
