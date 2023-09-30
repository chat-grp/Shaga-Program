use anchor_lang::prelude::*;
use crate::{RentalAccounts};
use crate::states::AffairState;

pub enum RentalTerminationAuthority {
    Clockwork,
    Client,
    TerminateAffair,
}

pub fn handler(ctx: Context<RentalAccounts>, termination_by: RentalTerminationAuthority) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;

    // Step 1: Calculate the actual time server was used (in hours)
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    let actual_time = (current_time - rental_account.rental_start_time) / 3600;
    let actual_payment = (actual_time * rental_account.rent_amount) as u64;

    // Step 3: Transfer the due payment to the lender (server)
    escrow_account.try_transfer_lamports(&ctx.accounts.lender, actual_payment)?;

    // Step 4: Refund the remaining balance to the client
    let refund_amount = escrow_account.locked_amount - actual_payment;
    if refund_amount > 0 {
        escrow_account.try_transfer_lamports(&ctx.accounts.client, refund_amount)?;
    }

    // Step 5: Update lender karma points based on who terminated the affair
    let lender_account = &mut ctx.accounts.lender;
    match termination_by {
        RentalTerminationAuthority::Clockwork | RentalTerminationAuthority::Client => lender_account.give_thumbs_up(),
        RentalTerminationAuthority::TerminateAffair => lender_account.give_thumbs_down(),
    }

    // Step 6: Update affair state to indicate it's Available
    affair_account.affair_state = AffairState::Available;
    affair_account.rental = None;

    Ok(())
}
