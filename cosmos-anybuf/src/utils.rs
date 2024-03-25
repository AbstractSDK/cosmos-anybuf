use cosmwasm_std::{
    to_json_vec, ContractResult, QuerierWrapper, StdError, StdResult, SystemResult,
};

use crate::anybuf_interface::{StargateQuery, StargateResponse};

/// Queries StargateQuery and decodes it to the StargateQueryResponse type
pub fn query_decode<R: StargateResponse, Q: StargateQuery>(
    querier: &QuerierWrapper,
    query: Q,
) -> StdResult<R> {
    let request = query.to_query();
    let raw = to_json_vec(&request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
    })?;
    match querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {system_err}"
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {contract_err}"
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => {
            R::from_buf(value.0).ok_or(StdError::generic_err(format!(
                "Failed to deserialize response to {}",
                std::any::type_name::<R>()
            )))
        }
    }
}
