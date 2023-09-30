use anchor_lang::prelude::*;

#[error_code]
pub enum ShagaErrorCode {
    #[msg("Invalid Session")]
    InvalidAffair,
    #[msg("Invalid Lender")]
    InvalidLender,
    #[msg("Invalid Payload")]
    InvalidPayload,
    #[msg("Sessions List Full")]
    AffairListFull,
    #[msg("Client Already in a Session")]
    ClientAlreadyInAffair,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Rental Termination Time")]
    InvalidRentalTerminationTime,
    #[msg("Invalid Termination Time")]
    InvalidTerminationTime,
    #[msg("Session Occupied")]
    AffairAlreadyJoined,
    #[msg("Thread Initialization Failed")]
    ThreadInitializationFailed,
}