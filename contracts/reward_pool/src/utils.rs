use cosmwasm_std::{MessageInfo, StdError, StdResult, Storage, Uint128};

use terraswap::asset::AssetInfoRaw;

use crate::state::CONFIGURATION;

pub fn get_received_native_fund(storage: &dyn Storage, info: MessageInfo) -> StdResult<Uint128> {
    let config = CONFIGURATION.load(storage)?;

    if info.funds.len() != 1u64 as usize {
        return Err(StdError::generic_err("invalid denom received"));
    }
    if let AssetInfoRaw::NativeToken { denom } = config.reward_asset_info {
        let amount: Uint128 = info
            .funds
            .iter()
            .find(|c| c.denom == *denom)
            .map(|c| Uint128::from(c.amount))
            .unwrap_or_else(Uint128::zero);
        Ok(amount)
    } else {
        Err(StdError::generic_err("not support denom reward"))
    }
}
