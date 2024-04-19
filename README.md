# Cosmos Anybuf

A low-level Rust protobuf crate for use in Stargate CosmWasm messages.

## Supported Modules

### Neutron

- Token Factory
- Interchain Queries
- Interchain Transactions

## Features

- Protobuf encoded messages
- Parsing of protobuf encoded responses (Query or Reply)

## Getting started

Currently cosmos-anybuf does not have crate published to crates.io, meaning it could only be installed as git dependency

```toml
[dependencies]
cosmos-anybuf = { version = "0.1.0", git = "https://github.com/AbstractSDK/cosmos-anybuf" }
```

## Example

### Sending Stargate messages

```rust
use cosmos_anybuf::{chains::neutron::Neutron, interfaces::TokenFactory};

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let sender = env.contract.address.to_string();
    let message = match msg {
        // Just call a method of the module you want to use
        ExecuteMsg::CreateDenom { subdenom } => Neutron::create_denom(sender, subdenom),
        // You can also specify module in case method names are repeated  
        ExecuteMsg::Mint {
            amount,
            mint_to_address,
        } => <Neutron as TokenFactory>::mint(sender, amount, mint_to_address),
    };

    Ok(Response::new().add_message(message))
    }
```

### Querying Stargate Message

```rust
use cosmos_anybuf::{chains::neutron::Neutron, interfaces::TokenFactory};

#[entry_point]
pub fn query_params(deps: Deps) -> StdResult<<Neutron as TokenFactory>::QueryParamsResponse> {
    let params = Neutron::query_params(&deps.querier)?;
    Ok(params)
}
```
