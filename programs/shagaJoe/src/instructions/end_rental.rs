use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;

#[derive(PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum RentalTerminationAuthority {
    Clockwork,
    Client,
    TerminateAffair,
}

#[derive(Accounts)]
pub struct EndRentalAccounts<'info> {
    /// checked below if signer == client or thread
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: checked below
    #[account(mut)]
    pub client: UncheckedAccount<'info>,
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: SystemAccount<'info>,
    #[account(mut)]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

/// can be done by either the client, clockwork, or affair authority
pub fn handle_ending_rental(
    ctx: Context<EndRentalAccounts>,
    termination_by: RentalTerminationAuthority,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;
    let system_program = &ctx.accounts.system_program;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let lender_account = &ctx.accounts.lender;
    let client = &ctx.accounts.client;
    let signer = &ctx.accounts.signer;

    // check if signer is the client
    if client.key() != signer.key() {
        // check if signer is thread. if it is not then fail early.
        if rental_account.rental_clockwork_thread_id != signer.key() {
            msg!("Invalid clockwork thread rental termination key.");
            return Err(ShagaErrorCode::InvalidTerminationTime.into());
        }
    }
    // fail early if rental does not exist
    if affair_account.rental.is_none() {
        msg!("No rental found. possibly already terminated or ended by the client.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }
    // Step 1: Calculate the actual time server was used (in hours)
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    let actual_time = (current_time - rental_account.rental_start_time) / 3600;
    let actual_payment = actual_time * rental_account.rent_amount;

    // Step 3: Transfer the due payment to the lender (server)
    solana_program::program::invoke_signed(
        &solana_program::system_instruction::transfer(
            &escrow_account.key(),
            &lender_account.key(),
            actual_payment,
        ),
        &[
            escrow_account.to_account_info().clone(),
            lender_account.to_account_info().clone(),
            system_program.to_account_info().clone(),
        ],
        &[], // TODO:
    )?;

    // Step 4: Refund the remaining balance to the client
    let refund_amount = escrow_account.locked_amount - actual_payment;
    if refund_amount > 0 {
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::transfer(
                &escrow_account.key(),
                &client.key(),
                refund_amount,
            ),
            &[
                escrow_account.to_account_info().clone(),
                client.to_account_info().clone(),
                system_program.to_account_info().clone(),
            ],
            &[], // TODO:
        )?;
    }

    // Step 5: Update lender karma points based on who terminated the affair
    let lender_account = &mut ctx.accounts.lender;
    match termination_by {
        RentalTerminationAuthority::Clockwork | RentalTerminationAuthority::Client => {
            lender_account.give_thumbs_up()
        }
        RentalTerminationAuthority::TerminateAffair => lender_account.give_thumbs_down(),
    }

    // Step 6: Update affair state to indicate it's Available
    affair_account.affair_state = AffairState::Available;
    affair_account.rental = None;

    // Step 7: Add Affair Back to Affair List
    let affair_pubkey = affair_account.key();
    affairs_list_account.register_affair(affair_pubkey)?;

    /* TODO:
    // Step 8: Transfer Remaining Lamports to Vault and Zero Out Rental Account
    let remaining_lamports = **rental_account.try_borrow_lamports()?;
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            rental_account.to_account_info().key,
            ctx.accounts.vault.to_account_info().key,
            remaining_lamports
        ),
        &[
            rental_account.to_account_info().clone(),
            ctx.accounts.vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ]
    )?;

    // Zero out the data in the rental account
    let mut data = rental_account.to_account_info().try_borrow_mut_data()?;
    for byte in data.iter_mut() {
        *byte = 0;
    }
    */
    Ok(())
}

// #[derive(Accounts)]
// pub struct CloseEndRentalAccounts<'info> {
//     pub authority: Signer<'info>,
//     #[account(mut, close = authority)]
//     pub rental: Account<'info, Rental>,
//     #[account(mut)]
//     pub recipient: AccountInfo<'info>,
// }
