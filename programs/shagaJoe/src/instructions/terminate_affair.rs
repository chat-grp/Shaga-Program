use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;

use solana_program::{clock::Clock, system_instruction};

#[derive(Accounts)]
pub struct TerminateAffairAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub client: SystemAccount<'info>,
    #[account(mut, has_one = authority @ ShagaErrorCode::UnauthorizedAffairCreation)]
    pub lender: Account<'info, Lender>,
    #[account(mut, seeds = [SEED_AFFAIR], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW, affair.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

impl<'info> TerminateAffairAccounts<'info> {
    // fn validate_termination_conditions(
    //     &self,
    //     termination_by: &AffairTerminationAuthority,
    // ) -> Result<()> {
    //     let current_time = Clock::get()?.unix_timestamp as u64;

    //     match termination_by {
    //         AffairTerminationAuthority::Clockwork => {
    //             if current_time < self.affair.affair_termination_time {
    //                 msg!("Affair cannot be terminated before the scheduled time.");
    //                 return Err(ShagaErrorCode::InvalidTerminationTime.into());
    //             }

    //             if self.affair.rental.is_some() {
    //                 msg!("Affair cannot be terminated by Clockwork if there's an active rental.");
    //                 return Err(ShagaErrorCode::InvalidTerminationTime.into());
    //             }

    //             if let TriggerContext::Timestamp { started_at } = self
    //                 .affair_clockwork_thread
    //                 .exec_context
    //                 .unwrap()
    //                 .trigger_context
    //             {
    //                 if current_time < started_at as u64 {
    //                     msg!("Clockwork Thread has not reached the trigger timestamp yet.");
    //                     return Err(ShagaErrorCode::InvalidTerminationTime.into());
    //                 }
    //             } else {
    //                 msg!("Invalid trigger context for Clockwork Thread.");
    //                 return Err(ShagaErrorCode::InvalidTerminationTime.into());
    //             }
    //         }
    //         AffairTerminationAuthority::Lender => {
    //             // The Server (Lender) is allowed to terminate the affair regardless of the affair_termination_time
    //         }
    //     }

    //     Ok(())
    // }
}

pub fn handle_affair_termination(ctx: Context<TerminateAffairAccounts>) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow = &ctx.accounts.escrow;
    let lender = &mut ctx.accounts.lender;
    let system_program = &ctx.accounts.system_program;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let vault = &ctx.accounts.vault;
    let client = &ctx.accounts.client;

    // clockwork termination is handled by another instruction (terminate_vacant_affair)
    // let affair_clockwork_thread = &ctx.accounts.affair_clockwork_thread;
    // Validate termination conditions
    // ctx.accounts
    //     .validate_termination_conditions(&termination_by)?;

    if let Some(_active_rental_pubkey) = affair_account.rental {
        // Step 1: Calculate the actual time server was used (in hours)
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;
        let actual_time = (current_time - affair_account.active_rental_start_time) / 3600;
        let actual_payment = actual_time * affair_account.usdc_per_hour as u64;

        // Step 2: Transfer the due payment to the lender (server)
        solana_program::program::invoke(
            &system_instruction::transfer(&escrow.key(), &lender.key(), actual_payment),
            &[
                escrow.to_account_info().clone(),
                lender.to_account_info().clone(),
                system_program.to_account_info().clone(),
            ],
        )?;

        // Step 3: Refund the remaining balance to the client
        let refund_amount = affair_account.due_rent_amount - actual_payment;
        if refund_amount > 0 {
            solana_program::program::invoke(
                &system_instruction::transfer(&escrow.key(), &client.key(), refund_amount),
                &[
                    escrow.to_account_info().clone(),
                    client.to_account_info().clone(),
                    system_program.to_account_info().clone(),
                ],
            )?;
        }

        lender.give_thumbs_down();
    }
    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // handled by anchor
    affair_account.close(vault.to_account_info())?;
    // Update the affair state to Terminated
    // affair_account.affair_state = AffairState::Unavailable;

    // Transfer remaining lamports to the vault
    // let remaining_lamports = **ctx
    //     .accounts
    //     .affair
    //     .to_account_info()
    //     .try_borrow_lamports()?;
    // solana_program::program::invoke(
    //     &solana_program::system_instruction::transfer(
    //         &affair_account.key(),
    //         &vault.key(),
    //         remaining_lamports,
    //     ),
    //     &[
    //         affair_account.to_account_info().clone(),
    //         vault.to_account_info().clone(),
    //         system_program.to_account_info().clone(),
    //     ],
    // )?;
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
