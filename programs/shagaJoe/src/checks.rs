use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use crate::states::{Session, Escrow};
use crate::errors::ShagaErrorCode;

pub fn check_can_start_rental(session: &Session) -> ProgramResult {
    if !session.can_join() {
        Err(ShagaErrorCode::InvalidSession.into())
    } else {
        Ok(())
    }
}

pub fn check_client_already_in_session(session: &Session, client_key: &Pubkey) -> ProgramResult {
    if session.client == *client_key {
        Err(ShagaErrorCode::ClientAlreadyInSession.into())
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
