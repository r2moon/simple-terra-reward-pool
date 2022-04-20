use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, from_binary, to_binary, Coin, Decimal, StdError, Uint128};

use cw20::Cw20ReceiveMsg;
use reward_pool::reward_pool::{Cw20HookMsg, ExecuteMsg, QueryMsg, State};
use terraswap::asset::AssetInfo;

use crate::{
    contract::{execute, query},
    state::STATE,
    tests::{mock_querier::mock_dependencies, test_utils::instantiate_reward_pool},
};

#[test]
fn fails_if_caller_is_not_funder() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info).unwrap();

    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "policy",
        &[Coin {
            denom: "uusd".to_string(),
            amount: Uint128::from(1u128),
        }],
    );

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("unauthorized"));
}

#[test]
fn fails_if_amount_is_zero() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "funder",
        &[Coin {
            denom: "uusd".to_string(),
            amount: Uint128::zero(),
        }],
    );

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("amount is zero"));
}

#[test]
fn fails_if_no_deposits() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "funder",
        &[Coin {
            denom: "uusd".to_string(),
            amount: Uint128::from(1u128),
        }],
    );

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("no deposits"));
}

#[test]
fn fails_if_multiple_denom_received() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "funder",
        &[
            Coin {
                denom: "ueur".to_string(),
                amount: Uint128::from(1u128),
            },
            Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(1u128),
            },
        ],
    );

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("invalid denom received"));
}

#[test]
fn fails_if_cw20_received() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "funder".to_string(),
        msg: to_binary(&Cw20HookMsg::Fund {}).unwrap(),
        amount: Uint128::from(1u128),
    });

    let info = mock_info("reward_token", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("invalid reward token"));
}

#[test]
fn fund_rewards() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let total_deposits = Uint128::from(100u128);
    let reward_amount = Uint128::from(1000000000u128);

    STATE
        .save(
            &mut deps.storage,
            &State {
                acc_per_share: Decimal::zero(),
                total_deposits,
            },
        )
        .unwrap();
    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "funder",
        &[Coin {
            denom: "uusd".to_string(),
            amount: reward_amount,
        }],
    );

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();

    let state_res: State = from_binary(&res).unwrap();

    assert_eq!(
        State {
            acc_per_share: Decimal::from_ratio(reward_amount, total_deposits),
            total_deposits,
        },
        state_res
    );
}

#[test]
fn increase_acc_per_share() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let total_deposits = Uint128::from(100u128);
    let reward_amount = Uint128::from(1000000000u128);
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
    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "funder",
        &[Coin {
            denom: "uusd".to_string(),
            amount: reward_amount,
        }],
    );

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();

    let state_res: State = from_binary(&res).unwrap();

    assert_eq!(
        State {
            acc_per_share: acc_per_share + Decimal::from_ratio(reward_amount, total_deposits),
            total_deposits,
        },
        state_res
    );
}

#[test]
fn return_correct_logs() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let total_deposits = Uint128::from(100u128);
    let reward_amount = Uint128::from(1000000000u128);

    STATE
        .save(
            &mut deps.storage,
            &State {
                acc_per_share: Decimal::zero(),
                total_deposits,
            },
        )
        .unwrap();
    let msg = ExecuteMsg::Fund {};

    let info = mock_info(
        "funder",
        &[Coin {
            denom: "uusd".to_string(),
            amount: reward_amount,
        }],
    );

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "fund"), attr("amount", reward_amount),]
    );
}
