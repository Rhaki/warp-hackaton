use cosmwasm_std::Addr;
use cw_storage_plus::{index_list, IndexedMap, Item, MultiIndex};

use crate::definitons::{Config, UserPosition};

pub const CONFIG: Item<Config> = Item::new("config_key");

#[index_list(UserPosition)]
pub struct UserPositionIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, UserPosition, u64>,
}

#[allow(non_snake_case)]
pub fn POSITIONS<'a>() -> IndexedMap<'a, u64, UserPosition, UserPositionIndexes<'a>> {
    let indexes = UserPositionIndexes {
        owner: MultiIndex::new(
            |_, val| val.owner.clone(),
            "ns_pools",
            "ns_pools_collateral",
        ),
    };

    IndexedMap::new("ns_pools", indexes)
}
