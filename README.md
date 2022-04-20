# Reward pool

## Overview
Users will stake cw20 token and get rewarded in selected reward token(denom or cw20 reward token).
Every time, user call `deposit`, `withdraw`, or `claim` msg, user's accumulated rewards will be updated.
When the funder fund reward token, the `acc_per_share` will be updated. This vaule is indicates how much reward is allocated per one staking token.
When there is no deposits, it is impossible to fund.

## Contract Msgs
### instantiate

```
pub struct InstantiateMsg {
    pub governance: String,
    pub funder: String,
    pub staking_token: String,
    pub reward_asset_info: AssetInfo,
}
```

- `governance` is a address who can update config.
- `funder` is a address who can fund reward token.
- `staking_token` is a cw20 token address which users will stake.
- `reward_asset_info` is a cw20 token or denom asset info which users will get reward.

### execute
```
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Withdraw {
        amount: Uint128,
    },
    Claim {},
    Fund {},
    UpdateConfig {
        governance: Option<String>,
        funder: Option<String>,
    },
}
```

- `Receive(Cw20ReceiveMsg)`
Cw20 token receive hook handler for staking or fund.
- `Withdraw`
Execute Msg for withdrawing staking token
- `Claim`
Execute Msg for claiming rewards
- `Fund`
Execute Msg for funding rewards - only funder can call.
- `UpdateConfig`
Execute Msg for updating config(governance and funder address) - only governance can call.

### query
```
pub enum QueryMsg {
    Config {},
    State {},
    UserInfo { user: String },
}
```

- `Config`
Query current configuration (governance, funder, staking token, reward token)
- `State`
Query current contract state (total deposits, reward acc per share)
- `UserInfo`
Query information for specific user (user's stake amount and pending rewards)
