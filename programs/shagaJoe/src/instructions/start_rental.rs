use anchor_lang::prelude::*;
use anchor_lang::prelude::Clock;
use anchor_spl::token;
use anchor_spl::token::Transfer;
use crate::{AffairState, Affair, StartRental, errors::ShagaErrorCode, ID, EndRental};
use crate::shaga::initialize_end_rental_thread;
use clockwork_sdk::state::{Thread, ThreadAccount};
use clockwork_sdk::cpi::{thread_create, ThreadCreate};
use anchor_lang::context::CpiContext;
use solana_program::instruction::Instruction;
use solana_program::native_token::LAMPORTS_PER_SOL;
use crate::instructions::TerminationAuthority;
use crate::seeds::SEED_AUTHORITY_THREAD;
use sha2::{Sha256, Digest};

pub fn handler(
    ctx: Context<StartRental>,
    rental_termination_time: i64,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;

    // Step 1: Verify that the transaction is signed by the client
    if !ctx.accounts.client.is_signer {
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
        rental_termination_time <= current_time {
        msg!("Invalid rental termination time.");
        return Err(ShagaErrorCode::InvalidRentalTerminationTime.into());
    }

    // Step 4: Calculate rent cost & fee amount
    let rental_duration_hours = (rental_termination_time - current_time) / 3600;
    let rent_amount = rental_duration_hours * affair_account.usdc_per_hour as i64;
    let fee_amount = rent_amount / 100; //TODO: EVALUATE ROUNDING ERRORS

    // Step 4A: Check balance
    let required_amount = rent_amount + fee_amount;
    if ctx.accounts.client_token_account.amount < required_amount as u64 {
        msg!("Insufficient funds in client's token account.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }

    // Step 5: Token Transfer Logic for Fee
    let cpi_accounts_fee = Transfer {
        from: ctx.accounts.client_token_account.to_account_info().clone(),
        to: ctx.accounts.vault.to_account_info().clone(),
        authority: ctx.accounts.client.to_account_info().clone(),
    };
    let cpi_program_fee = ctx.accounts.token_program.to_account_info().clone();
    let cpi_ctx_fee = CpiContext::new(cpi_program_fee, cpi_accounts_fee);
    token::transfer(cpi_ctx_fee, fee_amount as u64)?;

    // Step 6: Token Transfer Logic for Rent
    let cpi_accounts_rent = Transfer {
        from: ctx.accounts.client_token_account.to_account_info().clone(),
        to: ctx.accounts.escrow.to_account_info().clone(),
        authority: ctx.accounts.client.to_account_info().clone(),
    };
    let cpi_program_rent = ctx.accounts.token_program.to_account_info().clone();
    let cpi_ctx_rent = CpiContext::new(cpi_program_rent, cpi_accounts_rent);
    token::transfer(cpi_ctx_rent, (rent_amount - fee_amount) as u64)?;

    // Step 6A: Update locked amount flag
    escrow_account.locked_amount = (rent_amount - fee_amount) as u64;

    // Step 7: Mark the affair as joined by setting the rental account pubkey
    affair_account.join(*ctx.accounts.rental.to_account_info().key).expect("Failed to start rental");


    // Step 8: Initialize the Rental account
    rental_account.initialize(
        *ctx.accounts.client.to_account_info().key,
        *ctx.accounts.affair.to_account_info().key,
        (rent_amount - fee_amount) as u64,
        current_time,
        rental_termination_time,
        *ctx.accounts.clockwork_thread.to_account_info().key,
    );

    // Step 9: Save the rental termination time and Clockwork thread ID in the rental account
    rental_account.rental_termination_time = rental_termination_time as u64;

    // Convert EndRental accounts to AccountMeta for the instruction
    let end_rental_accounts = vec![
        ctx.accounts.client.to_account_info().to_account_meta(true),  // Signer
        ctx.accounts.client_token_account.to_account_info().to_account_meta(false),
        ctx.accounts.lender_token_account.to_account_info().to_account_meta(false),
        ctx.accounts.token_program.to_account_info().to_account_meta(false),
        ctx.accounts.affair.to_account_info().to_account_meta(false),
        ctx.accounts.lender.to_account_info().to_account_meta(false),
        ctx.accounts.escrow.to_account_info().to_account_meta(false),
        ctx.accounts.rental.to_account_info().to_account_meta(false),
        ctx.accounts.vault.to_account_info().to_account_meta(false),
        ctx.accounts.system_program.to_account_info().to_account_meta(false),
        ctx.accounts.clockwork_thread.to_account_info().to_account_meta(true),  // Signer
    ];

    let end_rental_instruction = Instruction {
        program_id: ID, // Your program ID
        accounts: end_rental_accounts,
        data: TerminationAuthority::Clockwork.try_to_vec()?,
    };

    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: rental_termination_time,
    };

    // To have a unique thread_id
    let mut hasher = Sha256::new();
    hasher.update(ctx.accounts.client.to_account_info().key.to_bytes());
    hasher.update(ctx.accounts.rental.to_account_info().key.to_bytes());
    let result = hasher.finalize();
    let thread_id: Vec<u8> = result.to_vec();

    let bump = *ctx.bumps.get("thread_authority").unwrap();

    let bump = *ctx.accounts.vault.bump();
    let cpi_ctx = clockwork_sdk::cpi::CpiContext::new_with_signer(
        ctx.accounts.clockwork_program.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: ctx.accounts.client.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            thread: ctx.accounts.clockwork_thread.to_account_info(),
            authority: ctx.accounts.client.to_account_info(),
        },
        &[&[SEED_AUTHORITY_THREAD, &[bump]]],
    );

    clockwork_sdk::cpi::thread_create(
        cpi_ctx,
        LAMPORTS_PER_SOL,
        thread_id.clone(),
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