use anchor_lang::prelude::*;

#[error_code]
pub enum ShagaErrorCode {
    #[msg("Invalid Session")]
    InvalidSession,
    #[msg("Invalid Lender")]
    InvalidLender,
    #[msg("Invalid Payload")]
    InvalidPayload,
    #[msg("Session List Full")]
    SessionListFull,
    #[msg("Client Already in Session")]
    ClientAlreadyInSession,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Rental Termination Time")]
    InvalidRentalTerminationTime,
}