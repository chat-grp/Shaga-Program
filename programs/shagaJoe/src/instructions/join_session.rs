use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::prelude::Clock;
use crate::{SessionState, JoinSession, Session, Lender, Escrow, errors::ShagaErrorCode};

pub fn handler(
    ctx: Context<JoinSession>,
    rent_amount: u64,
    rental_termination_time: i64,  // New parameter
) -> Result<()> {
    let session_account = &mut ctx.accounts.session;
    let lender_account = &mut ctx.accounts.lender;
    let escrow_account = &mut ctx.accounts.escrow;

    // Step 1: Verify that the transaction is signed by the client
    if !ctx.accounts.client.is_signer {
        msg!("Client must be the signer.");
        return Err(ShagaErrorCode::InvalidSession.into());
    }

    // Step 2: Validate if the session can be joined
    if !session_account.can_join() {
        msg!("Session cannot be joined.");
        return Err(ShagaErrorCode::InvalidSession.into());
    }

    // Step 3: Validate rental_termination_time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    if rental_termination_time > session_account.session_termination_time ||
        rental_termination_time <= current_time {
        msg!("Invalid rental termination time.");
        return Err(ShagaErrorCode::InvalidRentalTerminationTime.into());
    }

    // Step 4: Check if the client is already part of the session
    if session_account.client == *ctx.accounts.client.key {
        msg!("Client is already part of the session.");
        return Err(ShagaErrorCode::ClientAlreadyInSession.into());
    }

    // Step 5: Token Transfer Logic
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.client.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        rent_amount,
    )?;

    // Step 6: Validate that the escrow has the expected rent amount
    if escrow_account.locked_amount < rent_amount {
        msg!("Insufficient funds in escrow.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }

    // Step 7: Set the session start time
    session_account.rental_start_time = current_time;

    // Step 8: Deduct rent amount from escrow
    escrow_account.locked_amount -= rent_amount;

    // Step 9: Mark the session as joined by setting the client pubkey
    session_account.join(*ctx.accounts.client.key);

    // Step 10: Update lender state
    lender_account.increment_sessions();

    Ok(())
}
