use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Uint128};
use cw_storage_plus::{Item, Map};

use reward_pool::reward_pool::State;
use terraswap::asset::AssetInfoRaw;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub governance: CanonicalAddr,
    pub funder: CanonicalAddr,
    pub staking_token: CanonicalAddr,
    pub reward_asset_info: AssetInfoRaw,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct UserInfo {
    pub stake_amount: Uint128,
    pub pending_amount: Uint128,
    pub reward_debt: Uint128,
}

pub const CONFIGURATION: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const USER_INFO: Map<&[u8], UserInfo> = Map::new("user_infos");
