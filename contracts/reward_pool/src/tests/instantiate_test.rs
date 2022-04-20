use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{from_binary, Decimal, Uint128};

use reward_pool::reward_pool::{ConfigResponse, InstantiateMsg, QueryMsg, State};
use terraswap::asset::AssetInfo;

use crate::{
    contract::{instantiate, query},
    tests::mock_querier::mock_dependencies,
};

#[test]
fn instantiate_reward_pool() {
    let mut deps = mock_dependencies(&[]);

    let reward_asset_info = AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };

    let msg = InstantiateMsg {
        governance: String::from("governance"),
        funder: String::from("funder"),
        staking_token: String::from("staking_token"),
        reward_asset_info: reward_asset_info.clone(),
    };

    let info = mock_info("policy", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();

    let config_res: ConfigResponse = from_binary(&res).unwrap();

    assert_eq!(
        ConfigResponse {
            governance: String::from("governance"),
            funder: String::from("funder"),
            staking_token: String::from("staking_token"),
            reward_asset_info,
        },
        config_res
    );

    let res = query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap();

    let state_res: State = from_binary(&res).unwrap();

    assert_eq!(
        State {
            acc_per_share: Decimal::zero(),
            total_deposits: Uint128::zero(),
        },
        state_res
    );
}
