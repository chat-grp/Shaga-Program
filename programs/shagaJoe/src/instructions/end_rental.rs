use anchor_lang::prelude::*;
use crate::{RentalAccounts};
use crate::states::{AffairState, Rental};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum RentalTerminationAuthority {
    Clockwork,
    Client,
    TerminateAffair,
}

pub fn handler(ctx: Context<RentalAccounts>, termination_by: RentalTerminationAuthority) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;

    // Step 1: Calculate the actual time server was used (in hours) TODO: USE U64
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    let actual_time = (current_time - rental_account.rental_start_time) / 3600;
    let actual_payment = actual_time * rental_account.rent_amount;

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

    // Step 7: Add Affair Back to Affair List
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let affair_pubkey = *ctx.accounts.affair.to_account_info().key;
    affairs_list_account.register_affair(affair_pubkey)?;

    // Step 8: Transfer Remaining Lamports to Vault and Zero Out Rental Account
    let remaining_lamports = **ctx.accounts.rental.to_account_info().try_borrow_lamports()?;
    // Transfer remaining lamports to the vault
    ctx.accounts.rental.to_account_info().try_transfer_lamports(
        ctx.accounts.vault.to_account_info(),
        remaining_lamports
    )?;
    // Zero out the data in the rental account
    let mut data = ctx.accounts.rental.try_borrow_mut_data()?;
    for byte in data.iter_mut() {
        *byte = 0;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CloseRentalAccounts<'info> {
    pub authority: Signer<'info>,
    #[account(mut, close = authority)]
    pub rental: Account<'info, Rental>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}