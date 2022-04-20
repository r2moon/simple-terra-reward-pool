use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{from_binary, StdError};

use reward_pool::reward_pool::{ConfigResponse, ExecuteMsg, QueryMsg};
use terraswap::asset::AssetInfo;

use crate::{
    contract::{execute, query},
    tests::{mock_querier::mock_dependencies, test_utils::instantiate_reward_pool},
};

#[test]
fn fails_if_caller_is_not_governance() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        governance: Some(String::from("governance1")),
        funder: Some(String::from("funder1")),
    };

    let info = mock_info("policy", &[]);

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, StdError::generic_err("unauthorized"));
}

#[test]
fn update_governance() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        governance: Some(String::from("governance1")),
        funder: None,
    };

    let info = mock_info("governance", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();

    let config_res: ConfigResponse = from_binary(&res).unwrap();

    assert_eq!(
        ConfigResponse {
            governance: String::from("governance1"),
            funder: String::from("funder"),
            staking_token: String::from("staking_token"),
            reward_asset_info,
        },
        config_res
    );
}

#[test]
fn update_funder() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        governance: None,
        funder: Some(String::from("funder1")),
    };

    let info = mock_info("governance", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();

    let config_res: ConfigResponse = from_binary(&res).unwrap();

    assert_eq!(
        ConfigResponse {
            governance: String::from("governance"),
            funder: String::from("funder1"),
            staking_token: String::from("staking_token"),
            reward_asset_info,
        },
        config_res
    );
}

#[test]
fn update_governance_and_funder() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    instantiate_reward_pool(&mut deps, reward_asset_info.clone()).unwrap();

    let msg = ExecuteMsg::UpdateConfig {
        governance: Some(String::from("governance1")),
        funder: Some(String::from("funder1")),
    };

    let info = mock_info("governance", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();

    let config_res: ConfigResponse = from_binary(&res).unwrap();

    assert_eq!(
        ConfigResponse {
            governance: String::from("governance1"),
            funder: String::from("funder1"),
            staking_token: String::from("staking_token"),
            reward_asset_info,
        },
        config_res
    );
}
