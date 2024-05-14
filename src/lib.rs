// Define the contract module and its associated functionality.
mod contract;
// Publicly expose the messaging interfaces for external use.
pub mod msg;
// Define and manage the state of the contract.
mod state;
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
pub use cw721_base::{ContractError, InstantiateMsg, MinterResponse};
use {
    contract::Cw721MetadataContract,
    msg::{ExecuteMsg, QueryMsg},
};

// Define the entry point for contract instantiation.
#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Delegate the instantiation logic to the Cw721MetadataContract.
    Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)
}

// Define the entry point for executing contract messages.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Delegate the execution logic to the Cw721MetadataContract.
    Cw721MetadataContract::default().execute(deps, env, info, msg)
}

// Define the entry point for querying the contract.
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // Delegate the query processing to the Cw721MetadataContract.
    Cw721MetadataContract::default().query(deps, env, msg)
}
