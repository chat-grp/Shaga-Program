use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;

use solana_program::{clock::Clock, native_token::Sol};

#[derive(Accounts)]
pub struct TerminateAffairAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: checked below. possibly none.
    #[account(mut)]
    pub client: SystemAccount<'info>,
    #[account(mut, has_one = authority @ ShagaErrorCode::UnauthorizedAffairCreation, seeds = [SEED_LENDER, affair.authority.as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    #[account(mut, has_one = authority, seeds = [SEED_AFFAIR, authority.key().as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    #[account(mut, seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

pub fn handle_affair_termination(ctx: Context<TerminateAffairAccounts>) -> Result<()> {
    let affair_account = &ctx.accounts.affair;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &ctx.accounts.rental;
    let client = &ctx.accounts.client;
    let authority = &ctx.accounts.authority;
    let vault = &ctx.accounts.vault;

    if affair_account.rental.is_none() {
        msg!("Invalid instruction there is no ongoing rental.");
        return Err(ShagaErrorCode::InvalidTerminationInstruction.into());
    }
    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    if current_time >= rental_account.rental_termination_time {
        msg!("current time is higher than rental termination time. rental has ended.");
        let authority_account_info = &mut authority.to_account_info();
        let escrow_account_info = &mut escrow_account.to_account_info();

        let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
        let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
        let actual_payment = rental_account.rent_amount;
        **escrow_lamports -= actual_payment;
        **authority_lamports += actual_payment;

        msg!("actual_payment: {}", Sol(actual_payment));
        msg!("authority_lamports: {}", Sol(**authority_lamports));
        msg!("escrow_lamports: {}", Sol(**escrow_lamports));
    } else {
        // using a factor of 100:
        let scaling_factor = 100_u64;

        let actual_time = (current_time as f64 - rental_account.rental_start_time as f64) / 3600.0;
        let scaled_rental_duration = (actual_time * scaling_factor as f64) as u64;
        let actual_payment = scaled_rental_duration
            .checked_mul(affair_account.sol_per_hour)
            .ok_or(ShagaErrorCode::NumericalOverflow)?
            .checked_div(scaling_factor)
            .ok_or(ShagaErrorCode::NumericalOverflow)?;

        let refund_amount = escrow_account
            .locked_amount
            .checked_sub(actual_payment)
            .ok_or(ShagaErrorCode::NumericalOverflow)?;

        let client_account_info = &mut client.to_account_info();
        let authority_account_info = &mut authority.to_account_info();
        let escrow_account_info = &mut escrow_account.to_account_info();

        let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
        let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
        let mut client_lamports = client_account_info.try_borrow_mut_lamports()?;

        **escrow_lamports -= refund_amount + actual_payment;
        **authority_lamports += actual_payment;
        **client_lamports += refund_amount;

        msg!("actual_payment: {}", Sol(actual_payment));
        msg!("refund_amount: {}", Sol(refund_amount));
        msg!("escrow_lamports: {}", Sol(**escrow_lamports));
        msg!("authority_lamports: {}", Sol(**authority_lamports));
        msg!("client_lamports: {}", Sol(**client_lamports));

        let lender = &mut ctx.accounts.lender;
        lender.give_thumbs_down();
    }

    // check if lender has some sols to retrieve.
    let lender_account_info = &mut ctx.accounts.lender.to_account_info();
    let lender_rent = Rent::get()?.minimum_balance(lender_account_info.data_len());
    let lender_balance = lender_account_info.lamports() - lender_rent;
    if lender_balance > 0 {
        let authority_account_info = &mut ctx.accounts.authority.to_account_info();
        let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;

        **lender_lamports -= lender_balance;
        **authority_lamports += lender_balance;
    }

    // since rent ended and we already transfered the total.
    // we can close these accounts below.
    affair_account.close(vault.to_account_info())?;
    escrow_account.close(vault.to_account_info())?;
    rental_account.close(vault.to_account_info())?;

    Ok(())
}
