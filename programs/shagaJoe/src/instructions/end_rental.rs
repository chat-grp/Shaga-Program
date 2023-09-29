use anchor_lang::prelude::*;
use crate::{EndRental, affair, Lender, Escrow};
use crate::errors::ShagaErrorCode;
use anchor_spl::token::{self, Transfer, TokenAccount};
use crate::states::affairState;

pub enum TerminationAuthority {
    Clockwork,
    Client,
    Server,
}

pub fn handler(ctx: Context<EndRental>, termination_by: TerminationAuthority) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let lender_account = &mut ctx.accounts.lender;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;


    // Step 1: Calculate the actual time server was used (in hours)
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    let actual_time = (current_time - rental_account.rental_start_time) / 3600;
    let actual_payment = (actual_time * rental_account.rent_amount) as u64;


    // Step 2: Validate that the escrow has enough funds to cover the payment
    if escrow_account.locked_amount < actual_payment as u64 {
        msg!("Insufficient funds in escrow.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }

    // Step 3: Transfer the due payment to the lender (server)
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow.to_account_info().clone(),
        to: ctx.accounts.lender_token_account.to_account_info().clone(),
        authority: ctx.accounts.system_program.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, actual_payment as u64)?;

    // Step 4: Refund the remaining balance to the client
    let refund_amount = escrow_account.locked_amount - actual_payment;
    if refund_amount > 0 {
        let cpi_accounts_refund = Transfer {
            from: ctx.accounts.escrow.to_account_info().clone(),
            to: ctx.accounts.client_token_account.to_account_info().clone(),
            authority: ctx.accounts.system_program.to_account_info().clone(),
        };
        let cpi_program_refund = ctx.accounts.token_program.to_account_info().clone();
        let cpi_ctx_refund = CpiContext::new(cpi_program_refund, cpi_accounts_refund);
        token::transfer(cpi_ctx_refund, refund_amount)?;
    }

    // Step 5: Update lender karma points based on who terminated the affair
    match termination_by {
        TerminationAuthority::Clockwork | TerminationAuthority::Client => lender_account.give_thumbs_up(),
        TerminationAuthority::Server => lender_account.give_thumbs_down(),
    }

    // Step 6: Update affair state to indicate it's finished
    affair_account.affair_state = affairState::Finished { client: *ctx.accounts.client.key };

    Ok(())
}
