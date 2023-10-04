use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;

use solana_program::clock::Clock;

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
    /// CHECK: checked below. possibly none.
    #[account(mut, seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: AccountInfo<'info>, // Account<'info, Escrow>,
    /// CHECK: checked below. possibly none.
    #[account(mut, seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: AccountInfo<'info>,
    #[account(mut, seeds = [SEED_ESCROW], bump)]
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
    /// ending rental should only be handled if there is a rental
    fn handle_ending_rental(&self) -> Result<()> {
        // Step 1: Calculate the actual time server was used (in hours)
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;

        let rental = Rental::deserialize_data(&self.rental.data.borrow_mut())?;
        let escrow = Escrow::deserialize_data(&self.escrow.data.borrow_mut())?;

        let actual_time = (current_time - rental.rental_start_time) / 3600;
        let actual_payment = actual_time * rental.rent_amount;
        // Step 4: Refund the remaining balance to the client
        let refund_amount = escrow.locked_amount - actual_payment;

        let client_account_info = &mut self.client.to_account_info();
        let lender_account_info = &mut self.lender.to_account_info();
        let escrow_account_info = &mut self.escrow.to_account_info();

        let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;
        let mut client_lamports = client_account_info.try_borrow_mut_lamports()?;

        **escrow_lamports -= refund_amount + actual_payment;
        **lender_lamports += actual_payment;
        **client_lamports += refund_amount;
        // Step 3: Transfer the due payment to the lender (server)
        // solana_program::program::invoke_signed(
        //     &solana_program::system_instruction::transfer(
        //         &self.escrow.key(),
        //         &self.lender.key(),
        //         actual_payment,
        //     ),
        //     &[
        //         self.escrow.to_account_info().clone(),
        //         self.lender.to_account_info().clone(),
        //         self.system_program.to_account_info().clone(),
        //     ],
        //     &[],
        // )?;

        // if refund_amount > 0 {
        //     solana_program::program::invoke_signed(
        //         &solana_program::system_instruction::transfer(
        //             &self.escrow.key(),
        //             &self.client.key(),
        //             refund_amount,
        //         ),
        //         &[
        //             self.escrow.to_account_info().clone(),
        //             self.client.to_account_info().clone(),
        //             self.system_program.to_account_info().clone(),
        //         ],
        //         &[],
        //     )?;
        // }

        Ok(())
    }
}

pub fn handle_affair_termination(ctx: Context<TerminateAffairAccounts>) -> Result<()> {
    let affair_account = &ctx.accounts.affair;
    let escrow = &ctx.accounts.escrow;
    let lender = &ctx.accounts.lender;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let vault = &ctx.accounts.vault;
    let client = &ctx.accounts.client;

    // clockwork termination is handled by another instruction
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
        let refund_amount = affair_account.due_rent_amount - actual_payment;

        // Step 2: Transfer the due payment to the lender (server)
        // solana_program::program::invoke(
        //     &system_instruction::transfer(&escrow.key(), &lender.key(), actual_payment),
        //     &[
        //         escrow.to_account_info().clone(),
        //         lender.to_account_info().clone(),
        //         system_program.to_account_info().clone(),
        //     ],
        // )?;

        // // Step 3: Refund the remaining balance to the client
        // if refund_amount > 0 {
        //     solana_program::program::invoke(
        //         &system_instruction::transfer(&escrow.key(), &client.key(), refund_amount),
        //         &[
        //             escrow.to_account_info().clone(),
        //             client.to_account_info().clone(),
        //             system_program.to_account_info().clone(),
        //         ],
        //     )?;
        // }
        let client_account_info = &mut client.to_account_info();
        let lender_account_info = &mut lender.to_account_info();

        let mut escrow_lamports = escrow.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;
        let mut client_lamports = client_account_info.try_borrow_mut_lamports()?;

        **escrow_lamports -= refund_amount + actual_payment;
        **lender_lamports += actual_payment;
        **client_lamports += refund_amount;

        let lender = &mut ctx.accounts.lender;
        lender.give_thumbs_down();
    }
    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // handled by anchor
    affair_account.close(vault.to_account_info())?;

    if affair_account.rental.is_some() {
        ctx.accounts.handle_ending_rental()?;
    }
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
