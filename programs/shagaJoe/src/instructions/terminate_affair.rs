use anchor_lang::prelude::*;
use crate::{AffairAccounts, AffairState, errors::ShagaErrorCode, ID, RentalAccounts};
use solana_program::clock::Clock;
use clockwork_sdk::state::{Thread, TriggerContext};
use crate::instructions::end_rental::*;
use crate::states::Affair;

pub enum AffairTerminationAuthority {
    Clockwork,
    Lender,
}

// To invoke end_rental
use crate::instructions::handler as end_rental_handler;
use crate::seeds::{SEED_ESCROW};


pub fn handler(
    ctx: Context<AffairAccounts>,
    termination_by: AffairTerminationAuthority
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let clockwork_thread = &ctx.accounts.clockwork_thread;

    // Validate termination conditions
    validate_termination_conditions(affair_account, clockwork_thread, termination_by)?;

    // If Lender is the authority and there's an active rental, terminate it
    if termination_by == AffairTerminationAuthority::Lender {
        if let Some(active_rental_pubkey) = affair_account.rental {
            // Construct RentalAccounts context from AffairAccounts
            let rental_accounts_context = construct_rental_context_from_affair(&ctx, active_rental_pubkey, &ID)?;

            // Call the function to terminate the rental
            end_rental_handler(rental_accounts_context, RentalTerminationAuthority::TerminateAffair)?;
        }
    }
    // Remove the affair from the list of active affairs
    // TODO: Remove the affair from the AffairList account
    // affairs_list.remove_affair(*affair_account.to_account_info().key);

    // Update the affair state to Terminated
    affair_account.affair_state = AffairState::Unavailable;

    // Optionally, close the affair account here if that is part of your logic
    // TODO: Close the affair account if required

    Ok(())
}

fn construct_rental_context_from_affair(
    ctx: &Context<AffairAccounts>,
    active_rental_pubkey: Pubkey,
    program_id: &Pubkey,
) -> Result<Context<RentalAccounts>> {

    // Step 1: Fetch Already Available Accounts
    let client = ctx.accounts.authority.to_account_info().clone();
    let affair = ctx.accounts.affair.clone();
    let lender = ctx.accounts.lender.to_account_info().clone();
    let system_program = ctx.accounts.system_program.clone();
    let clockwork_thread = ctx.accounts.clockwork_thread.to_account_info().clone();

    // Step 2: Derive PDAs
    let (escrow, _bump_escrow) = Pubkey::find_program_address(&[SEED_ESCROW, lender.key.as_ref(), client.key.as_ref()], program_id);
    let (vault, _bump_vault) = Pubkey::find_program_address(&[SEED_ESCROW], program_id);

    // Step 3: Create a mutable RentalAccounts instance
    let mut rental_accounts = RentalAccounts {
        client,
        affair,
        lender,
        escrow: escrow.into(),
        rental: active_rental_pubkey.into(),
        vault: vault.into(),
        system_program,
        clockwork_thread,
    };

    // Step 4: Construct RentalAccounts Context
    let rental_accounts_context = Context {
        program_id: &ID,
        accounts: &mut rental_accounts,
        remaining_accounts: &[],
        bumps: Default::default(),
    };

    // Step 5: Return
    Ok(rental_accounts_context)
}

fn validate_termination_conditions(
    affair_account: &Affair,
    clockwork_thread: &Thread,
    termination_by: AffairTerminationAuthority,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;

    match termination_by {
        AffairTerminationAuthority::Clockwork => {
            if current_time < affair_account.affair_termination_time {
                msg!("Affair cannot be terminated before the scheduled time.");
                return Err(ShagaErrorCode::InvalidTerminationTime.into());
            }

            if affair_account.rental.is_some() {
                msg!("Affair cannot be terminated by Clockwork if there's an active rental.");
                return Err(ShagaErrorCode::InvalidTerminationTime.into());
            }

            if let TriggerContext::Timestamp { started_at } = clockwork_thread.exec_context.unwrap().trigger_context {
                if current_time < started_at {
                    msg!("Clockwork Thread has not reached the trigger timestamp yet.");
                    return Err(ShagaErrorCode::InvalidTerminationTime.into());
                }
            } else {
                msg!("Invalid trigger context for Clockwork Thread.");
                return Err(ShagaErrorCode::InvalidTerminationTime.into());
            }
        },
        AffairTerminationAuthority::Lender => {
            // The Server (Lender) is allowed to terminate the affair regardless of the affair_termination_time
        },
    }

    Ok(())
}