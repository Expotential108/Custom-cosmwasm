use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Addr, Uint128};


#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub denom: String,
    pub cw20_id: u64,
    pub cw20_label: String,
    pub token_name: String,
    pub token_symbol: String,
    pub proxy_id: u64,
    pub proxy_label: String,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Register {},
    Stake { validator: String },
    Unstake { validator: String, amount : Uint128 },
    ChangeValidator {from: String, to: String, amount: Uint128},
    Claim {},
    GetReward {},
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ConfigInfo {},
    Staked {address: Addr},
    TokenInfo {address: Addr},
}
