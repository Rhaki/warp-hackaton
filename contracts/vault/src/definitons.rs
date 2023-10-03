use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct Config {
    pub base_denom: String,
    pub quote_denom: String,
    pub counter_position: u64,
    pub astroport_pool_contract: Addr,
}

#[cw_serde]
pub struct UserPosition {
    pub owner: Addr,
    pub lower_bound: Decimal,
    pub uper_bound: Decimal,
    pub delta: Decimal,
    pub base_amount: Uint128,
    pub quote_amount: Uint128,
}
