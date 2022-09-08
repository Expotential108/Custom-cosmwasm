
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::{Map, Item};
use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct ConfigInfo {
    pub denom: String,
    pub reward_denom: String,
    pub reward_contract : String,
    pub stake_contract_id : u64,
    pub stake_contract_label: String,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct StakeInfo {
    pub staked : Vec<Staked>,
    pub stake_contract : String,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq,  )]
pub struct Staked {
    pub amount: Uint128,
    pub validator: String,

}

pub const STAKEINFO : Map<&Addr, StakeInfo> = Map::new("stakeinfo");
pub const CONFIG : Item<ConfigInfo> = Item::new("delegateinfo");