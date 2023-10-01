use anchor_lang::prelude::*;
use crate::errors::ShagaErrorCode;
use crate::states::{Affair, Escrow};
use solana_program::entrypoint::ProgramResult;

pub fn check_can_start_rental(affair: &Affair) -> ProgramResult {
    if !affair.can_join() {
        msg!("The is already rented");
        return Err(ShagaErrorCode::AffairAlreadyJoined.into());
    } else {
        Ok(())
    }
}

pub fn check_client_already_in_affair(affair: &Affair, client_key: &Pubkey) -> ProgramResult {
    if affair.client == *client_key {
        msg!("Client already has rental active");
        return Err(ShagaErrorCode::ClientAlreadyInAffair.into());
    } else {
        Ok(())
    }
}

pub fn check_sufficient_funds_in_escrow(escrow: &Escrow, rent_amount: u64) -> ProgramResult {
    if escrow.locked_amount < rent_amount {
        msg!("Insufficient funds.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    } else {
        Ok(())
    }
}
