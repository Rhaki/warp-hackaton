use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Addr, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub base_denom: String,
    pub quote_denom: String,
    pub astroport_pool_contract: String,
    pub warp_controller: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePosition(CreatePositionMsg),
    Trigger(TriggerMsg),
}

#[cw_serde]
pub enum QueryMsg {
    GetPositions {
        user: String,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct CreatePositionMsg {
    pub owner: Addr,
    pub lower_bound: Decimal,
    pub uper_bound: Decimal,
    pub delta: Decimal,
}

#[cw_serde]
pub struct TriggerMsg {
    pub id: u64,
}
