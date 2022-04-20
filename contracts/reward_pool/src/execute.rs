use cosmwasm_std::{attr, Addr, Decimal, DepsMut, Response, StdError, StdResult, Uint128};

use terraswap::asset::{Asset, AssetInfo};

use crate::state::{CONFIGURATION, STATE, USER_INFO};

pub fn deposit(deps: DepsMut, user: String, amount: Uint128) -> StdResult<Response> {
    if amount.is_zero() {
        return Err(StdError::generic_err("amount is zero"));
    }

    let mut state = STATE.load(deps.storage)?;
    let mut user_info = USER_INFO
        .load(deps.storage, deps.api.addr_canonicalize(&user)?.as_slice())
        .unwrap_or_default();
    let new_reward = user_info.stake_amount * state.acc_per_share - user_info.reward_debt;
    user_info.pending_amount += new_reward;
    user_info.stake_amount += amount;
    user_info.reward_debt = user_info.stake_amount * state.acc_per_share;

    USER_INFO.save(
        deps.storage,
        deps.api.addr_canonicalize(&user)?.as_slice(),
        &user_info,
    )?;

    state.total_deposits += amount;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![attr("action", "deposit"), attr("amount", amount)]))
}

pub fn withdraw(deps: DepsMut, user: String, amount: Uint128) -> StdResult<Response> {
    if amount.is_zero() {
        return Err(StdError::generic_err("amount is zero"));
    }

    let config = CONFIGURATION.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;
    let mut user_info = USER_INFO
        .load(deps.storage, deps.api.addr_canonicalize(&user)?.as_slice())
        .unwrap_or_default();
    let new_reward = user_info.stake_amount * state.acc_per_share - user_info.reward_debt;
    user_info.pending_amount += new_reward;
    user_info.stake_amount -= amount;
    user_info.reward_debt = user_info.stake_amount * state.acc_per_share;

    USER_INFO.save(
        deps.storage,
        deps.api.addr_canonicalize(&user)?.as_slice(),
        &user_info,
    )?;

    state.total_deposits -= amount;
    STATE.save(deps.storage, &state)?;

    let asset: Asset = Asset {
        info: AssetInfo::Token {
            contract_addr: deps.api.addr_humanize(&config.staking_token)?.to_string(),
        },
        amount,
    };

    Ok(Response::new()
        .add_attributes(vec![attr("action", "withdraw"), attr("amount", amount)])
        .add_message(asset.into_msg(&deps.querier, Addr::unchecked(user))?))
}

pub fn claim(deps: DepsMut, user: String) -> StdResult<Response> {
    let config = CONFIGURATION.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;
    let mut user_info = USER_INFO
        .load(deps.storage, deps.api.addr_canonicalize(&user)?.as_slice())
        .unwrap_or_default();
    let new_reward = user_info.stake_amount * state.acc_per_share - user_info.reward_debt;
    let pending_reward = user_info.pending_amount + new_reward;
    user_info.pending_amount = Uint128::zero();
    user_info.reward_debt = user_info.stake_amount * state.acc_per_share;

    USER_INFO.save(
        deps.storage,
        deps.api.addr_canonicalize(&user)?.as_slice(),
        &user_info,
    )?;

    if pending_reward.is_zero() {
        return Err(StdError::generic_err("reward is zero"));
    }

    let asset: Asset = Asset {
        info: config.reward_asset_info.to_normal(deps.api)?,
        amount: pending_reward,
    };

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "claim"),
            attr("amount", pending_reward),
        ])
        .add_message(asset.into_msg(&deps.querier, Addr::unchecked(user))?))
}

pub fn fund(deps: DepsMut, funder: String, amount: Uint128) -> StdResult<Response> {
    let config = CONFIGURATION.load(deps.storage)?;

    if deps.api.addr_humanize(&config.funder)?.to_string() != funder {
        return Err(StdError::generic_err("unauthorized"));
    }

    if amount.is_zero() {
        return Err(StdError::generic_err("amount is zero"));
    }

    let mut state = STATE.load(deps.storage)?;
    if state.total_deposits.is_zero() {
        return Err(StdError::generic_err("no deposits"));
    }
    state.acc_per_share = state.acc_per_share + Decimal::from_ratio(amount, state.total_deposits);
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attributes(vec![attr("action", "fund"), attr("amount", amount)]))
}

pub fn update_config(
    deps: DepsMut,
    sender: String,
    governance: Option<String>,
    funder: Option<String>,
) -> StdResult<Response> {
    let mut config = CONFIGURATION.load(deps.storage)?;

    if deps.api.addr_humanize(&config.governance)?.to_string() != sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    if let Some(governance) = governance {
        config.governance = deps.api.addr_canonicalize(&governance)?;
    }

    if let Some(funder) = funder {
        config.funder = deps.api.addr_canonicalize(&funder)?;
    }

    CONFIGURATION.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
