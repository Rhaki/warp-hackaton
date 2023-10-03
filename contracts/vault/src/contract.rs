use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use rhaki_cw_plus::traits::IntoAddr;

use crate::{
    definitons::Config,
    execute::{run_create_position, run_trigger},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    response::ContractResponse,
    state::CONFIG,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResponse {
    CONFIG.save(
        deps.storage,
        &Config {
            base_denom: msg.base_denom,
            quote_denom: msg.quote_denom,
            counter_position: 0,
            astroport_pool_contract: msg.astroport_pool_contract.into_addr(deps.api)?,
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResponse {
    match msg {
        ExecuteMsg::CreatePosition(msg) => run_create_position(deps, info, msg),
        ExecuteMsg::Trigger(msg) => run_trigger(deps, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
