use cosmwasm_std::{Response, StdError};
use thiserror::Error;

pub type ContractResponse = Result<Response, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Wrong coins recived")]
    WrongAssetReceived {},

    #[error("Price not in range")]
    PriceNotInRange {},
}
