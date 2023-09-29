use anchor_lang::prelude::*;

#[error_code]
pub enum ShagaErrorCode {
    #[msg("Invalid affair")]
    InvalidAffair,
    #[msg("Invalid Lender")]
    InvalidLender,
    #[msg("Invalid Payload")]
    InvalidPayload,
    #[msg("affair List Full")]
    AffairListFull,
    #[msg("Client Already in affair")]
    ClientAlreadyInAffair,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Rental Termination Time")]
    InvalidRentalTerminationTime,
    #[msg("Invalid Termination Time")]
    InvalidTerminationTime,
}