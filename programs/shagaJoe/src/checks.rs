use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use crate::states::{affair, Escrow};
use crate::errors::ShagaErrorCode;

pub fn check_can_start_rental(affair: &affair) -> ProgramResult {
    if !affair.can_join() {
        Err(ShagaErrorCode::InvalidAffair.into())
    } else {
        Ok(())
    }
}

pub fn check_client_already_in_affair(affair: &affair, client_key: &Pubkey) -> ProgramResult {
    if affair.client == *client_key {
        Err(ShagaErrorCode::ClientAlreadyInAffair.into())
    } else {
        Ok(())
    }
}

pub fn check_sufficient_funds_in_escrow(escrow: &Escrow, rent_amount: u64) -> ProgramResult {
    if escrow.locked_amount < rent_amount {
        Err(ShagaErrorCode::InsufficientFunds.into())
    } else {
        Ok(())
    }
}
