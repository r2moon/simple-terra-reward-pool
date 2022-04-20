use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
use cosmwasm_std::{to_binary, OwnedDeps, StdResult, Uint128};

use crate::{
    contract::{execute, instantiate},
    tests::mock_querier::WasmMockQuerier,
};
use cw20::Cw20ReceiveMsg;
use reward_pool::reward_pool::{Cw20HookMsg, ExecuteMsg, InstantiateMsg};
use terraswap::asset::AssetInfo;

pub fn instantiate_reward_pool(
    deps: &mut OwnedDeps<MockStorage, MockApi, WasmMockQuerier>,
    reward_asset_info: AssetInfo,
) -> StdResult<()> {
    let msg = InstantiateMsg {
        governance: String::from("governance"),
        funder: String::from("funder"),
        staking_token: String::from("staking_token"),
        reward_asset_info: reward_asset_info.clone(),
    };

    let info = mock_info("policy", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();

    Ok(())
}

pub fn deposit(
    deps: &mut OwnedDeps<MockStorage, MockApi, WasmMockQuerier>,
    amount: Uint128,
) -> StdResult<()> {
    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr".to_string(),
        msg: to_binary(&Cw20HookMsg::Deposit {}).unwrap(),
        amount,
    });

    let info = mock_info("staking_token", &[]);

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    Ok(())
}
