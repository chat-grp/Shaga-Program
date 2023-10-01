use crate::states::Affair;
use crate::{errors::ShagaErrorCode, AffairAccounts, AffairState};
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use clockwork_sdk::state::{Thread, TriggerContext};
use solana_program::clock::Clock;
use solana_program::system_instruction;

#[derive(PartialEq, BorshSerialize, BorshDeserialize)]
pub enum AffairTerminationAuthority {
    Clockwork,
    Lender,
}

pub fn handler(
    ctx: Context<AffairAccounts>,
    termination_by: AffairTerminationAuthority,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let affair_clockwork_thread = &ctx.accounts.affair_clockwork_thread;

    // Validate termination conditions
    validate_termination_conditions(affair_account, affair_clockwork_thread, &termination_by)?;

    if termination_by == AffairTerminationAuthority::Lender {
        if let Some(_active_rental_pubkey) = affair_account.rental {
            // Step 1: Calculate the actual time server was used (in hours)
            let clock = Clock::get()?;
            let current_time = clock.unix_timestamp as u64;
            let actual_time = (current_time - affair_account.active_rental_start_time) / 3600;
            let actual_payment = actual_time * affair_account.usdc_per_hour as u64;

            // Step 2: Transfer the due payment to the lender (server)
            let lender_key = ctx.accounts.lender.to_account_info().key;
            let escrow_key = ctx.accounts.escrow.to_account_info().key; // Note the corrected field
            solana_program::program::invoke(
                &system_instruction::transfer(escrow_key, lender_key, actual_payment),
                &[
                    ctx.accounts.escrow.to_account_info().clone(),
                    ctx.accounts.lender.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                ],
            )?;

            // Step 3: Refund the remaining balance to the client
            let refund_amount = affair_account.due_rent_amount - actual_payment;
            if refund_amount > 0 {
                solana_program::program::invoke(
                    &system_instruction::transfer(
                        escrow_key,
                        ctx.accounts.client.to_account_info().key,
                        refund_amount,
                    ),
                    &[
                        ctx.accounts.escrow.to_account_info().clone(),
                        ctx.accounts.client.to_account_info().clone(),
                        ctx.accounts.system_program.to_account_info().clone(),
                    ],
                )?;
            }
        }
    }
    // Remove the affair from the list of active affairs
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // Update the affair state to Terminated
    affair_account.affair_state = AffairState::Unavailable;

    // Transfer remaining lamports to the vault
    let remaining_lamports = **ctx
        .accounts
        .affair
        .to_account_info()
        .try_borrow_lamports()?;
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            ctx.accounts.affair.to_account_info().key,
            ctx.accounts.vault.to_account_info().key,
            remaining_lamports,
        ),
        &[
            ctx.accounts.affair.to_account_info().clone(),
            ctx.accounts.vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    // Zero out the data in the affair account
    // let mut data = ctx
    //     .accounts
    //     .affair
    //     .to_account_info()
    //     .try_borrow_mut_data()?;
    // for byte in data.iter_mut() {
    //     *byte = 0;
    // }

    Ok(())
}

fn validate_termination_conditions(
    affair_account: &Affair,
    affair_clockwork_thread: &Thread,
    termination_by: &AffairTerminationAuthority,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;

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

            if let TriggerContext::Timestamp { started_at } = affair_clockwork_thread
                .exec_context
                .unwrap()
                .trigger_context
            {
                if current_time < started_at as u64 {
                    msg!("Clockwork Thread has not reached the trigger timestamp yet.");
                    return Err(ShagaErrorCode::InvalidTerminationTime.into());
                }
            } else {
                msg!("Invalid trigger context for Clockwork Thread.");
                return Err(ShagaErrorCode::InvalidTerminationTime.into());
            }
        }
        AffairTerminationAuthority::Lender => {
            // The Server (Lender) is allowed to terminate the affair regardless of the affair_termination_time
        }
    }

    Ok(())
}

// #[derive(Accounts)]
// pub struct CloseAffairAccounts<'info> {
//     pub authority: Signer<'info>,
//     #[account(mut, close = authority)]
//     pub affair: Account<'info, Affair>,
//     #[account(mut)]
//     pub recipient: AccountInfo<'info>,
// }
