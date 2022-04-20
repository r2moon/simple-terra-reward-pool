use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{
    attr, from_binary, BankMsg, Coin, CosmosMsg, Decimal, StdError, SubMsg, Uint128,
};

use reward_pool::reward_pool::{ExecuteMsg, QueryMsg, State, UserInfoResponse};
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
fn fails_if_nothing_to_claim() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::Claim {};

    let info = mock_info("addr", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("reward is zero"));
}

#[test]
fn update_user_info() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let stake_amount = Uint128::from(100u128);

    deposit(&mut deps, stake_amount).unwrap();

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

    let msg = ExecuteMsg::Claim {};

    let info = mock_info("addr", &[]);

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
            stake_amount: stake_amount,
            pending_amount: Uint128::zero(),
        },
        user_info_res
    );
}

#[test]
fn transfer_reward_token() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let stake_amount = Uint128::from(100u128);

    deposit(&mut deps, stake_amount).unwrap();

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

    let msg = ExecuteMsg::Claim {};

    let info = mock_info("addr", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        res.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: String::from("addr"),
            amount: vec![Coin {
                denom: "uusd".to_string(),
                amount: acc_per_share * stake_amount
            }]
        })),]
    );
}

#[test]
fn return_correct_logs() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let tax_rate = Decimal::from_ratio(1u128, 1000u128);
    deps.querier.with_tax(tax_rate, &[]);

    let stake_amount = Uint128::from(100u128);

    deposit(&mut deps, stake_amount).unwrap();

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

    let msg = ExecuteMsg::Claim {};

    let info = mock_info("addr", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let reward_without_tax = acc_per_share * stake_amount;
    let reward_with_tax = reward_without_tax - reward_without_tax * tax_rate;

    assert_eq!(
        res.attributes,
        vec![attr("action", "claim"), attr("amount", reward_with_tax),]
    );
}
