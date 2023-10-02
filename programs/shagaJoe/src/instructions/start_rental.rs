use crate::instructions::RentalTerminationAuthority;
use crate::{errors::*, seeds::*, states::*, ID};

use anchor_lang::prelude::*;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
};

#[derive(Accounts)]
pub struct StartRentalAccounts<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut)]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(init, payer = client, space = Escrow::size(), seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = client, space = Rental::size(), seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    #[account(signer)]
    pub rental_clockwork_thread: Account<'info, clockwork_sdk::state::Thread>,
}

pub fn handle_starting_rental(
    ctx: Context<StartRentalAccounts>,
    rental_termination_time: u64,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;
    let client_account = &mut ctx.accounts.client;

    // Step 1: Verify that the transaction is signed by the client
    if !client_account.is_signer {
        msg!("Client must be the signer.");
        return Err(ShagaErrorCode::InvalidAffair.into());
    }

    // Step 2: Validate if the affair can be joined
    if !affair_account.can_join() {
        msg!("Affair cannot be joined.");
        return Err(ShagaErrorCode::InvalidAffair.into());
    }

    // Step 3: Validate rental_termination_time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    if rental_termination_time > affair_account.affair_termination_time
        || rental_termination_time <= current_time as u64
    {
        msg!("Invalid rental termination time.");
        return Err(ShagaErrorCode::InvalidRentalTerminationTime.into());
    }

    // Step 4: Calculate rent cost & fee amount
    let rental_duration_hours = (rental_termination_time - current_time) / 3600;
    let rent_amount = rental_duration_hours * affair_account.usdc_per_hour as u64;
    let fee_amount = rent_amount / 100; //TODO: EVALUATE ROUNDING ERRORS

    // Step 4A: Check balance in terms of Lamports
    if client_account.lamports() < (rent_amount + fee_amount) as u64 {
        msg!("Insufficient funds.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }

    // Step 5: Transfer fee to the vault
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            client_account.key,
            &ctx.accounts.vault.key(),
            fee_amount as u64,
        ),
        &[
            client_account.to_account_info().clone(),
            ctx.accounts.vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    // Step 6: Transfer the rent to the escrow
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            client_account.key,
            &escrow_account.key(),
            (rent_amount - fee_amount) as u64,
        ),
        &[
            client_account.to_account_info().clone(),
            escrow_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    // Step 6A: Update locked amount flag in Escrow
    escrow_account.locked_amount = (rent_amount - fee_amount) as u64;

    // Step 7: Mark the affair as joined by setting the rental account pubkey
    affair_account
        .join(rental_account.key())
        .expect("Failed to start rental");

    // Step 8: Initialize the Rental account
    rental_account.initialize(
        ctx.accounts.client.key(),
        affair_account.key(),
        (rent_amount - fee_amount) as u64,
        current_time as u64,
        rental_termination_time,
        ctx.accounts.rental_clockwork_thread.key(),
    );

    // Step 9A: Accounts for instruction
    let end_rental_accounts = vec![
        AccountMeta::new_readonly(ctx.accounts.client.key(), true), // Signer
        AccountMeta::new(affair_account.key(), false),
        AccountMeta::new(ctx.accounts.lender.key(), false),
        AccountMeta::new(ctx.accounts.escrow.key(), false),
        AccountMeta::new(rental_account.key(), false),
        AccountMeta::new(ctx.accounts.vault.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.rental_clockwork_thread.key(), true), // Signer
    ];
    // Step 9B: Instruction
    let end_rental_instruction = Instruction {
        program_id: ID,
        accounts: end_rental_accounts,
        data: RentalTerminationAuthority::Clockwork.try_to_vec()?,
    };
    // Step 9C: Thread Trigger & Thread_ID
    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: rental_termination_time as i64,
    };
    let (thread_id, bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            ctx.accounts.client.key().as_ref(),
            affair_account.key().as_ref(),
        ],
        ctx.program_id,
    );
    let cpi_signer: &[&[u8]] = &[SEED_THREAD, &[bump]];
    let binding_seeds = &[cpi_signer];

    let my_cpi_context = anchor_lang::context::CpiContext::new_with_signer(
        ctx.accounts.rental_clockwork_thread.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: ctx.accounts.client.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            thread: ctx.accounts.rental_clockwork_thread.to_account_info(),
            authority: ctx.accounts.client.to_account_info(),
        },
        binding_seeds,
    );

    let thread_id_vec: Vec<u8> = thread_id.to_bytes().to_vec();

    clockwork_sdk::cpi::thread_create(
        my_cpi_context, // Use the CPI context you've created
        LAMPORTS_PER_SOL,
        thread_id_vec, // Use the converted thread_id
        vec![end_rental_instruction.into()],
        trigger,
    )?;

    // Step 9B: Save the end_rental thread ID in the rental account
    rental_account.rental_clockwork_thread_id = thread_id.into();

    // Step 10: increment affairs
    let lender_account = &mut ctx.accounts.lender;
    lender_account.increment_affairs();

    // Step 11: Associate the rental account with the affair
    affair_account.rental = Some(rental_account.key());

    // Step 12: Remove Affair from Affair List
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // Step 13: Update the Affair account
    affair_account.active_rental_start_time = current_time;
    affair_account.due_rent_amount = (rent_amount - fee_amount) as u64;
    //affair_account.active_locked_amount = (rent_amount - fee_amount) as u64;

    Ok(())
}

/*
#[derive(Accounts)]
pub struct InitializeThread<'info> {
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: AccountInfo<'info>,
    #[account(mut, address = solana_program::pubkey::Pubkey(thread_authority.key().to_bytes().to_vec(), active_rental.key().to_bytes().to_vec()))]
    pub thread: Account<'info, clockwork_sdk::state::Thread>,
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: Account<'info, clockwork_sdk::state::Thread>,
    pub active_rental: AccountInfo<'info>,
}


#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct StartThread<'info> {
    #[account(
    seeds = [b"highscore_list_v2".as_ref()],
    bump,
    )]
    pub highscore: Account<'info, Highscore>,
    #[account(
    seeds = [b"price_pool".as_ref()],
    bump,
    )]
    pub price_pool: Account<'info, Pricepool>,
    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    /// The signer who will pay to initialize the program.
    /// (not to be confused with the thread executions).
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    /// Address to assign to the newly created thread.
    #[account(mut, address = Thread::pubkey(thread_authority.key(), thread_id))]
    pub thread: SystemAccount<'info>,
    /// The pda that will own and manage the thread.
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}
*/
