use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use cw::set_contract_version;

use sei_cosmwasm::{SeiMsg, SeiQueryWrapper};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<SeiMsg>> {
    cw2::set_contract_version(deps.storage, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;
    Ok(Response::new())
}

