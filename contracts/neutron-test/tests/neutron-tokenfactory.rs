use cosmwasm_std::CosmosMsg;
use cw_orch::daemon::sender::Sender;
use cw_orch::daemon::{self, Wallet};
use cw_orch::prelude::*;
use cw_orch::state::ChainState;
use cw_orch::testing::bank::BankModule;
use cw_orch::{
    contract::{artifacts_dir_from_workspace, interface_traits::Uploadable},
    daemon::Daemon,
    environment::TxHandler,
    interchain::{ChannelCreator, InterchainEnv, Starship},
    tokio::runtime::{Handle, Runtime},
};

struct NeutronApp {}

impl Uploadable for NeutronApp {
    fn wasm(&self) -> cw_orch::prelude::WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("neutron_test")
            .unwrap()
    }
}

trait CosmosStargateToAny {
    fn to_any(self) -> prost_types::Any;
}

impl CosmosStargateToAny for CosmosMsg {
    fn to_any(self) -> prost_types::Any {
        match self {
            CosmosMsg::Stargate { type_url, value } => prost_types::Any {
                type_url,
                value: value.0,
            },
            _ => unreachable!(),
        }
    }
}

struct NeutronSetup {
    neutron: Daemon,
    juno: Daemon,
    addr: Addr,
}

fn setup(handle: Handle) -> NeutronSetup {
    env_logger::init();
    let starship = Starship::new(handle.clone(), None).unwrap();

    let interchain = starship.interchain_env();

    let local_neutron: Daemon = interchain.chain("neutron-1").unwrap();

    let cosmwasm_client: cw_orch::daemon::queriers::CosmWasm = local_neutron.query_client();
    let res = handle
        .block_on(async { cosmwasm_client.params().await })
        .unwrap();
    println!("{res:?}");
    let neutron_sender = local_neutron.sender();
    println!("sender: {neutron_sender}");
    let local_juno: Daemon = interchain.chain("juno-1").unwrap();
    // Upload and instantiate
    let upload_resp = local_juno.upload(&NeutronApp {}).unwrap();
    let code_id = upload_resp.uploaded_code_id().unwrap();
    let instantiate_resp = local_juno
        .instantiate(
            code_id,
            &cosmwasm_std::Empty {},
            Some("neutron_test"),
            None,
            &[],
        )
        .unwrap();
    let addr = instantiate_resp.instantiated_contract_address().unwrap();
    NeutronSetup {
        neutron: local_neutron,
        juno: local_juno,
        addr,
    }
}

mod tx {
    use super::*;

    use cw_orch::anyhow;
    use neutron_test::msg::{ExecuteMsg, QueryMsg};

    const SUBDENOM: &str = "ntrn";
    const MINT_AMOUNT: u128 = 65535;

    // Testing everything together so it doesn't overload grpc
    #[test]
    fn test_all() {
        let rt = Runtime::new().unwrap();

        let NeutronSetup {
            neutron,
            juno: _,
            addr,
        } = setup(rt.handle().clone());

        let subdenom = String::from(SUBDENOM);

        create_denom(&neutron, &addr, subdenom.clone()).unwrap();

        mint(
            &neutron,
            &addr,
            Coin {
                denom: format!("factory/{addr}/{subdenom}"),
                amount: MINT_AMOUNT.into(),
            },
            addr.to_string(),
        )
        .unwrap();

        let admin: String = neutron
            .query(
                &QueryMsg::DenomAuthorityMetadata {
                    creator: addr.to_string(),
                    subdenom: subdenom.clone(),
                },
                &addr,
            )
            .unwrap();
        assert_eq!(admin, addr);
    }

    fn create_denom(
        neutron: &Daemon,
        contract_address: &Addr,
        subdenom: String,
    ) -> anyhow::Result<()> {
        neutron.execute(&ExecuteMsg::CreateDenom { subdenom }, &[], contract_address)?;
        Ok(())
    }

    fn mint(
        neutron: &Daemon,
        contract_address: &Addr,
        amount: Coin,
        mint_to_address: String,
    ) -> anyhow::Result<()> {
        neutron.execute(
            &ExecuteMsg::Mint {
                amount,
                mint_to_address,
            },
            &[],
            contract_address,
        )?;
        Ok(())
    }
}
