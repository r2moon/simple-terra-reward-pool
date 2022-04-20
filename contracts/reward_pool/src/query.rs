use cosmwasm_std::{Deps, StdResult};

use reward_pool::reward_pool::{ConfigResponse, State, UserInfoResponse};

use crate::state::{CONFIGURATION, STATE, USER_INFO};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIGURATION.load(deps.storage)?;

    Ok(ConfigResponse {
        governance: deps.api.addr_humanize(&config.governance)?.to_string(),
        funder: deps.api.addr_humanize(&config.funder)?.to_string(),
        staking_token: deps.api.addr_humanize(&config.staking_token)?.to_string(),
        reward_asset_info: config.reward_asset_info.to_normal(deps.api)?,
    })
}

pub fn query_state(deps: Deps) -> StdResult<State> {
    Ok(STATE.load(deps.storage)?)
}

pub fn query_user_info(deps: Deps, user: String) -> StdResult<UserInfoResponse> {
    let user_info = USER_INFO
        .load(deps.storage, deps.api.addr_canonicalize(&user)?.as_slice())
        .unwrap_or_default();

    let state = STATE.load(deps.storage)?;

    Ok(UserInfoResponse {
        stake_amount: user_info.stake_amount,
        pending_amount: (user_info.stake_amount * state.acc_per_share - user_info.reward_debt)
            + user_info.pending_amount,
    })
}
