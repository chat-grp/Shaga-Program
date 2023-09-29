use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::prelude::Clock;
use crate::{affairState, StartRental, affair, Lender, Escrow, errors::ShagaErrorCode};

pub fn handler(
    ctx: Context<StartRental>,
    rental_termination_time: i64,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let lender_account = &mut ctx.accounts.lender;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;

    // Step 1: Verify that the transaction is signed by the client
    if !ctx.accounts.client.is_signer {
        msg!("Client must be the signer.");
        return Err(ShagaErrorCode::Invalidaffair.into());
    }

    // Step 2: Validate if the affair can be joined
    if !affair_account.can_join() {
        msg!("affair cannot be joined.");
        return Err(ShagaErrorCode::Invalidaffair.into());
    }

    // Step 3: Validate rental_termination_time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    if rental_termination_time > affair_account.affair_termination_time ||
        rental_termination_time <= current_time {
        msg!("Invalid rental termination time.");
        return Err(ShagaErrorCode::InvalidRentalTerminationTime.into());
    }

    // Step 3A: Calculate rent duration
    let rental_duration_hours = (rental_termination_time - current_time) / 3600;  // Assuming time is in seconds

    // Step 3B: Calculate expected rent_amount based on affair rate and duration
    let rent_amount = rental_duration_hours * affair_account.usdc_per_hour as i64;

    // Step 4: Token Transfer Logic
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.client.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        rent_amount as u64,
    )?;

    // Step 5: Initialize the escrow account with the rent amount
    escrow_account.locked_amount = rent_amount as u64;

    // Step 6: Validate that the escrow has the expected rent amount
    if escrow_account.locked_amount < rent_amount as u64 {
        msg!("Insufficient funds in escrow.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }

// Step 7: Initialize the Rental account
    rental_account.initialize(
        *ctx.accounts.client.to_account_info().key,
        *ctx.accounts.affair.to_account_info().key,
        rent_amount as u64,
        current_time,
        rental_termination_time,
        *ctx.accounts.rental_clockwork_thread.to_account_info().key  // Fixed this line
    );

    // Step 8: Update the associated affair account to reflect that it's now active
    affair_account.affair_state = affairState::Active;

    // Step 8: Deduct rent amount from escrow
    escrow_account.locked_amount -= rent_amount;

    // Step 9: Mark the affair as joined by setting the client pubkey
    affair_account.join(*ctx.accounts.client.key);

    // Step 10: Update lender state
    lender_account.increment_affairs();

    // Step 11: Save the rental termination time and Clockwork thread ID in the affair account
    rental_account.rental_termination_time = rental_termination_time;
    rental_account.rental_clockwork_thread_id = *ctx.accounts.rental_clockwork_thread.key;

    Ok(())
}