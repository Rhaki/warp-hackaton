use std::collections::HashMap;

use astroport::asset::Asset;
use cosmwasm_std::{Coin, Decimal, DepsMut, MessageInfo, Response, StdError, Uint128, WasmMsg};
use rhaki_cw_plus::{
    math::IntoDecimal,
    wasm::{CosmosMsgBuilder, WasmMsgBuilder},
};

use crate::{
    definitons::UserPosition,
    msg::{CreatePositionMsg, TriggerMsg},
    querier::astroport_price,
    response::{ContractError, ContractResponse},
    state::{CONFIG, POSITIONS},
};

pub fn run_create_position(
    deps: DepsMut,
    info: MessageInfo,
    msg: CreatePositionMsg,
) -> ContractResponse {
    let mut config = CONFIG.load(deps.storage)?;

    let mut map_coin: HashMap<String, Uint128> = HashMap::new();

    for coin in info.funds {
        if coin.denom == config.base_denom || coin.denom == config.quote_denom {
            map_coin.insert(coin.denom, coin.amount);
        }
    }

    config.counter_position += 1;

    POSITIONS().save(
        deps.storage,
        config.counter_position,
        &UserPosition {
            owner: info.sender,
            lower_bound: msg.lower_bound,
            uper_bound: msg.uper_bound,
            delta: msg.delta,
            base_amount: map_coin.get(&config.base_denom).unwrap().to_owned(),
            quote_amount: map_coin.get(&config.quote_denom).unwrap().to_owned(),
        },
    )?;

    CONFIG.save(deps.storage, &config)?;

    let price = astroport_price(deps.as_ref())?;

    let current_ratio = Decimal::from_ratio(
        map_coin.get(&config.base_denom).unwrap().clone(),
        map_coin.get(&config.quote_denom).unwrap().clone(),
    );

    if current_ratio + msg.delta > price && current_ratio < msg.delta + price {
        return Err(ContractError::PriceNotInRange {});
    }

    Ok(Response::new())
}

pub fn run_trigger(deps: DepsMut, msg: TriggerMsg) -> ContractResponse {
    let config = CONFIG.load(deps.storage)?;
    let info = POSITIONS().load(deps.storage, msg.id)?;

    let _price = astroport_price(deps.as_ref())?;

    let current_price = Decimal::from_ratio(info.base_amount, info.quote_amount);

    let total_balance = current_price * info.base_amount + info.quote_amount;

    let per = Decimal::from_ratio(
        current_price * info.base_amount,
        current_price * info.base_amount + info.quote_amount,
    );

    let target_price =
        (info.uper_bound - info.lower_bound) * (Decimal::one() - per) + info.lower_bound;

    let buy_price = target_price - info.delta;
    let sell_price = target_price + info.delta;

    let msg: Result<WasmMsg, StdError> = if target_price > sell_price {
        let amount = total_balance * (info.delta / current_price);
        WasmMsg::build_execute(
            config.astroport_pool_contract,
            &astroport::pair::ExecuteMsg::Swap {
                offer_asset: Asset {
                    info: astroport::asset::AssetInfo::NativeToken {
                        denom: config.quote_denom.clone(),
                    },
                    amount: amount.clone(),
                },
                ask_asset_info: Some(astroport::asset::AssetInfo::NativeToken {
                    denom: config.base_denom,
                }),
                belief_price: Some(current_price),
                max_spread: Some("0.05".into_decimal()),
                to: None,
            },
            vec![Coin::new(amount.u128(), config.quote_denom)],
        )
    } else if target_price < buy_price {
        let amount = total_balance * (info.delta);
        WasmMsg::build_execute(
            config.astroport_pool_contract,
            &astroport::pair::ExecuteMsg::Swap {
                offer_asset: Asset {
                    info: astroport::asset::AssetInfo::NativeToken {
                        denom: config.base_denom.clone(),
                    },
                    amount,
                },
                ask_asset_info: Some(astroport::asset::AssetInfo::NativeToken {
                    denom: config.quote_denom,
                }),
                belief_price: Some(current_price),
                max_spread: Some("0.05".into_decimal()),
                to: None,
            },
            vec![Coin::new(amount.u128(), config.base_denom)],
        )
    } else {
        Err(StdError::generic_err("no price valide"))
    };

    Ok(Response::new().add_message(msg?.into_cosmos_msg()))
}
