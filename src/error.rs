use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("End height must be greater than the current block height")]
    InvalidEndHeight {},

    #[error("End height must be greater than the current block height")]
    InvalidStartHeight {},

    #[error("Invalid coordinates, must be within the width and height of the grid")]
    InvalidCoordinates {},

    #[error("Invalid color, color code must be between 0 and 15")]
    InvalidColor {},

    #[error("This address is still on cooldown, please wait until you can draw again")]
    StillOnCooldown {},
    
    #[error("The end height of this grid has been reached, drawing is no longer allowed")]
    EndHeightReached {},

    #[error("The start height of this grid has not been reached yet, drawing is disabled")]
    StartHeightNotReached {},

    #[error{"This address does not own any tokens from the collection address"}]
    InvalidCollectionStatus {},
}
