use crate::states::AffairState;
use crate::RentalAccounts;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum RentalTerminationAuthority {
    Clockwork,
    Client,
    TerminateAffair,
}

pub fn handler(
    ctx: Context<RentalAccounts>,
    termination_by: RentalTerminationAuthority,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;

    // Step 1: Calculate the actual time server was used (in hours)
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    let actual_time = (current_time - rental_account.rental_start_time) / 3600;
    let actual_payment = actual_time * rental_account.rent_amount;

    // Step 3: Transfer the due payment to the lender (server)
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            escrow_account.to_account_info().key,
            ctx.accounts.lender.to_account_info().key,
            actual_payment,
        ),
        &[
            escrow_account.to_account_info().clone(),
            ctx.accounts.lender.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    // Step 4: Refund the remaining balance to the client
    let refund_amount = escrow_account.locked_amount - actual_payment;
    if refund_amount > 0 {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                escrow_account.to_account_info().key,
                ctx.accounts.client.key,
                refund_amount,
            ),
            &[
                escrow_account.to_account_info().clone(),
                ctx.accounts.client.clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
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
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let affair_pubkey = *ctx.accounts.affair.to_account_info().key;
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
// pub struct CloseRentalAccounts<'info> {
//     pub authority: Signer<'info>,
//     #[account(mut, close = authority)]
//     pub rental: Account<'info, Rental>,
//     #[account(mut)]
//     pub recipient: AccountInfo<'info>,
// }
