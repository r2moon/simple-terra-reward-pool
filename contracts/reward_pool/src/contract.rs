#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use cw20::Cw20ReceiveMsg;
use reward_pool::reward_pool::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, State,
};
use terraswap::asset::AssetInfoRaw;

use crate::{
    execute::{claim, deposit, fund, update_config, withdraw},
    query::{query_config, query_state, query_user_info},
    state::{Config, CONFIGURATION, STATE},
    utils::get_received_native_fund,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    CONFIGURATION.save(
        deps.storage,
        &Config {
            governance: deps.api.addr_canonicalize(&msg.governance)?,
            funder: deps.api.addr_canonicalize(&msg.funder)?,
            staking_token: deps.api.addr_canonicalize(&msg.staking_token)?,
            reward_asset_info: msg.reward_asset_info.to_raw(deps.api)?,
        },
    )?;

    STATE.save(deps.storage, &State::default())?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    let sender = info.sender.to_string();
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, sender, amount),
        ExecuteMsg::Claim {} => claim(deps, sender),
        ExecuteMsg::Fund {} => {
            let amount = get_received_native_fund(deps.storage, info)?;
            fund(deps, sender, amount)
        }
        ExecuteMsg::UpdateConfig { governance, funder } => {
            update_config(deps, sender, governance, funder)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::UserInfo { user } => to_binary(&query_user_info(deps, user)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let config = CONFIGURATION.load(deps.storage)?;
    match from_binary(&cw20_msg.msg)? {
        Cw20HookMsg::Deposit {} => {
            if deps.api.addr_humanize(&config.staking_token)? == info.sender.clone() {
                return deposit(deps, cw20_msg.sender, cw20_msg.amount);
            }
            Err(StdError::generic_err("invalid staking token"))
        }
        Cw20HookMsg::Fund {} => {
            if let AssetInfoRaw::Token { contract_addr } = config.reward_asset_info {
                if deps.api.addr_humanize(&contract_addr)? == info.sender.clone() {
                    return fund(deps, cw20_msg.sender, cw20_msg.amount);
                }
            }
            Err(StdError::generic_err("invalid reward token"))
        }
    }
}
