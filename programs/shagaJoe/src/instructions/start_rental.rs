use anchor_lang::prelude::*;
use crate::{ RentalAccounts, errors::ShagaErrorCode, ID};
use anchor_lang::solana_program::{
    instruction::Instruction, native_token::LAMPORTS_PER_SOL };
use crate::instructions::RentalTerminationAuthority;
use crate::seeds::{ SEED_THREAD};


pub fn handler(
    ctx: Context<RentalAccounts>,
    rental_termination_time: i64,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;
    let client_account = &mut ctx.accounts.client;
    let clockwork_program = clockwork_sdk::ThreadProgram;

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
    let current_time = clock.unix_timestamp;
    if rental_termination_time > affair_account.affair_termination_time ||
        rental_termination_time <= current_time as i64 {
        msg!("Invalid rental termination time.");
        return Err(ShagaErrorCode::InvalidRentalTerminationTime.into());
    }

    // Step 4: Calculate rent cost & fee amount
    let rental_duration_hours = (rental_termination_time - current_time) / 3600;
    let rent_amount = rental_duration_hours * affair_account.usdc_per_hour as i64;
    let fee_amount = rent_amount / 100; //TODO: EVALUATE ROUNDING ERRORS

    // Step 4A: Check balance in terms of Lamports
    if client_account.lamports() < (rent_amount + fee_amount) as u64 {
        msg!("Insufficient funds.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }

    // Step 5: Transfer fee to the vault
    client_account.try_transfer_lamports(&ctx.accounts.vault, fee_amount as u64)?;

    // Step 6: Transfer the rent to the escrow
    client_account.try_transfer_lamports(escrow_account, (rent_amount - fee_amount) as u64)?;

    // Step 6A: Update locked amount flag in Escrow
    escrow_account.locked_amount = (rent_amount - fee_amount) as u64;

    // Step 7: Mark the affair as joined by setting the rental account pubkey
    affair_account.join(*ctx.accounts.rental.to_account_info().key).expect("Failed to start rental");


    // Step 8: Initialize the Rental account
    rental_account.initialize(
        *ctx.accounts.client.to_account_info().key,
        *ctx.accounts.affair.to_account_info().key,
        (rent_amount - fee_amount) as u64,
        current_time as i64,
        rental_termination_time,
        *ctx.accounts.clockwork_thread.to_account_info().key,
    );

    // Step 9A: Accounts for instruction
    let end_rental_accounts = vec![
        ctx.accounts.client.to_account_info().to_account_meta(true),  // Signer
        ctx.accounts.affair.to_account_info().to_account_meta(false),
        ctx.accounts.lender.to_account_info().to_account_meta(false),
        ctx.accounts.escrow.to_account_info().to_account_meta(false),
        ctx.accounts.rental.to_account_info().to_account_meta(false),
        ctx.accounts.vault.to_account_info().to_account_meta(false),
        ctx.accounts.system_program.to_account_info().to_account_meta(false),
        ctx.accounts.clockwork_thread.to_account_info().to_account_meta(true),  // Signer
    ];
    // Step 9B: Instruction
    let end_rental_instruction = Instruction {
        program_id: ID,
        accounts: end_rental_accounts,
        data: RentalTerminationAuthority::Clockwork.try_to_vec()?,
    };
    // Step 9C: Thread Trigger & Thread_ID
    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: rental_termination_time,
    };
    let (thread_id, bump) = Pubkey::find_program_address(
        &[SEED_THREAD, ctx.accounts.client.to_account_info().key.as_ref(), ctx.accounts.affair.to_account_info().key.as_ref()],
        ctx.program_id
    );

    // TODO: why is it "clockwork_sdk::cpi::CpiContext: and not "anchor_lang::context::CpiContext"??
    let my_cpi_context= clockwork_sdk::cpi::CpiContext::new_with_signer(
        clockwork_program.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: ctx.accounts.client.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            thread: ctx.accounts.clockwork_thread.to_account_info(),
            authority: ctx.accounts.client.to_account_info(),
        },
        &[&[SEED_THREAD, &[bump]]],
    );

    let thread_id_vec: Vec<u8> = thread_id.to_bytes().to_vec();

    clockwork_sdk::cpi::thread_create(
        my_cpi_context,  // Use the CPI context you've created
        LAMPORTS_PER_SOL,
        thread_id_vec,   // Use the converted thread_id
        vec![end_rental_instruction.into()],
        trigger,
    )?;

    // Step 9B: Save the end_rental thread ID in the rental account
    rental_account.rental_clockwork_thread_id = thread_id.into();

    // Step 10: increment affairs
    let lender_account = &mut ctx.accounts.lender;
    lender_account.increment_affairs();

    // Step 11: Associate the rental account with the affair
    affair_account.rental = Some(*ctx.accounts.rental.to_account_info().key);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeThread<'info> {
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    #[account(mut, address = clockwork_sdk::state::Thread::pubkey(thread_authority.key(), thread_id))]
    pub thread: Account<'info, clockwork_sdk::state::Thread>,
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: Account<'info, clockwork_sdk::state::Thread>,
    pub active_rental: Option<Pubkey>,
}

/*
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