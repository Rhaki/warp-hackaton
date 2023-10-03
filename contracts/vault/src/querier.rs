use std::collections::HashMap;

use astroport::{asset::AssetInfo, factory::PairsResponse, pair::PoolResponse};
use cosmwasm_std::{Decimal, Deps, StdResult, Uint128};

use crate::state::CONFIG;

pub fn astroport_price(deps: Deps) -> StdResult<Decimal> {
    let config = CONFIG.load(deps.storage)?;

    let mut map_amounts: HashMap<String, Uint128> = HashMap::new();

    let result: PoolResponse = deps.querier.query_wasm_smart(
        config.astroport_pool_contract,
        &astroport::pair::QueryMsg::Pool {},
    )?;

    for asset in result.assets {
        if let AssetInfo::NativeToken { denom } = asset.info {
            map_amounts.insert(denom, asset.amount);
        }
    }

    return Ok(Decimal::from_ratio(
        map_amounts.get(&config.base_denom).unwrap().clone(),
        map_amounts.get(&config.quote_denom).unwrap().clone(),
    ));
}
