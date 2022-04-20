use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, from_binary, to_binary, Decimal, StdError, Uint128};

use cw20::Cw20ReceiveMsg;
use reward_pool::reward_pool::{Cw20HookMsg, ExecuteMsg, QueryMsg, State, UserInfoResponse};
use terraswap::asset::AssetInfo;

use crate::{
    contract::{execute, query},
    state::STATE,
    tests::{
        mock_querier::mock_dependencies,
        test_utils::{deposit, instantiate_reward_pool},
    },
};

#[test]
fn fails_if_amount_is_zero() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr".to_string(),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
        amount: Uint128::zero(),
    });

    let info = mock_info("staking_token", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("amount is zero"));
}

#[test]
fn increase_total_deposits() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let stake_amount = Uint128::from(100u128);
    let total_deposits = Uint128::from(100u128);

    STATE
        .save(
            &mut deps.storage,
            &State {
                acc_per_share: Decimal::zero(),
                total_deposits,
            },
        )
        .unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr".to_string(),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
        amount: stake_amount,
    });

    let info = mock_info("staking_token", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();

    let state_res: State = from_binary(&res).unwrap();

    assert_eq!(
        State {
            acc_per_share: Decimal::zero(),
            total_deposits: total_deposits + stake_amount,
        },
        state_res
    );
}

#[test]
fn update_user_info() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let stake_amount = Uint128::from(100u128);
    let total_deposits = Uint128::from(100u128);
    let acc_per_share = Decimal::percent(50);

    STATE
        .save(
            &mut deps.storage,
            &State {
                acc_per_share,
                total_deposits,
            },
        )
        .unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr".to_string(),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
        amount: stake_amount,
    });

    let info = mock_info("staking_token", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UserInfo {
            user: "addr".to_string(),
        },
    )
    .unwrap();

    let user_info_res: UserInfoResponse = from_binary(&res).unwrap();

    assert_eq!(
        UserInfoResponse {
            stake_amount,
            pending_amount: Uint128::zero(),
        },
        user_info_res
    );
}

#[test]
fn update_pending_reward() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let stake_amount0 = Uint128::from(100u128);

    deposit(&mut deps, stake_amount0).unwrap();

    let stake_amount = Uint128::from(100u128);
    let total_deposits = Uint128::from(100u128);
    let acc_per_share = Decimal::percent(50);

    STATE
        .save(
            &mut deps.storage,
            &State {
                acc_per_share,
                total_deposits,
            },
        )
        .unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr".to_string(),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
        amount: stake_amount,
    });

    let info = mock_info("staking_token", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::UserInfo {
            user: "addr".to_string(),
        },
    )
    .unwrap();

    let user_info_res: UserInfoResponse = from_binary(&res).unwrap();

    assert_eq!(
        UserInfoResponse {
            stake_amount: stake_amount + stake_amount0,
            pending_amount: stake_amount0 * acc_per_share,
        },
        user_info_res
    );
}

#[test]
fn return_correct_logs() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let stake_amount = Uint128::from(100u128);
    let total_deposits = Uint128::from(100u128);
    let acc_per_share = Decimal::percent(50);

    STATE
        .save(
            &mut deps.storage,
            &State {
                acc_per_share,
                total_deposits,
            },
        )
        .unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr".to_string(),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
        amount: stake_amount,
    });

    let info = mock_info("staking_token", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "deposit"), attr("amount", stake_amount),]
    );
}
