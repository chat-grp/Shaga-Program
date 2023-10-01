use anchor_lang::prelude::*;
use clockwork_sdk::cpi::{thread_create};
use crate::{AffairAccounts, states::AffairState, ID};
use solana_program::instruction::Instruction;
use solana_program::native_token::LAMPORTS_PER_SOL;
use crate::errors::ShagaErrorCode;
use crate::instructions::AffairTerminationAuthority;
use crate::seeds::{SEED_THREAD, SEED_AUTHORITY_THREAD};
use solana_program::instruction::AccountMeta;


#[derive(anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]
pub struct AffairPayload {
    pub ip_address: [u8; 15],
    pub cpu_name: [u8; 64],
    pub gpu_name: [u8; 64],
    pub total_ram_mb: u32,
    pub usdc_per_hour: u32,
    pub affair_termination_time: u64,
}

impl Default for AffairPayload {
    fn default() -> Self {
        Self {
            ip_address: [0u8; 15],
            cpu_name: [0u8; 64],
            gpu_name: [0u8; 64],
            total_ram_mb: 0,
            usdc_per_hour: 0,
            affair_termination_time: 0,
        }
    }
}


pub fn handler(
    ctx: Context<AffairAccounts>,
    payload: AffairPayload,
) -> Result<()> {
    // Step 1: Initialize mutable references for Affair and Lender accounts
    let affair_account = &mut ctx.accounts.affair;

    // Step 2A: Populate it with payload and default values
    affair_account.lender = *ctx.accounts.creator.unsigned_key();
    affair_account.ip_address = payload.ip_address;
    affair_account.cpu_name = payload.cpu_name;
    affair_account.gpu_name = payload.gpu_name;
    affair_account.total_ram_mb = payload.total_ram_mb;
    affair_account.usdc_per_hour = payload.usdc_per_hour;
    affair_account.affair_termination_time = payload.affair_termination_time as u64;
    affair_account.affair_state = AffairState::Available;

    // Step 2B: Accounts for terminate_affair instruction
    let terminate_affair_accounts = vec![
        AccountMeta::new_readonly(*ctx.accounts.creator.to_account_info().key, true),
        AccountMeta::new(*ctx.accounts.affair.to_account_info().key, false),
        AccountMeta::new(*ctx.accounts.lender.to_account_info().key, false),
        AccountMeta::new_readonly(*ctx.accounts.system_program.to_account_info().key, false),
        AccountMeta::new_readonly(*ctx.accounts.affair_clockwork_thread.to_account_info().key, true),
    ];



    // Step 2C: Create the terminate_affair_instruction
    let terminate_affair_instruction = Instruction {
        program_id: ID,
        accounts: terminate_affair_accounts,
        data: AffairTerminationAuthority::Clockwork.try_to_vec()?
    };

    // Step 3: Fetch the current timestamp and validate affair termination time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    if affair_account.affair_termination_time <= current_time {
        msg!("Affair termination time must be in the future.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }

    // Step 5: Define thread_id with seeds & trigger for the termination thread
    let (thread_id, bump) = Pubkey::find_program_address(
        &[SEED_THREAD, ctx.accounts.creator.to_account_info().key.as_ref(), ctx.accounts.affair.to_account_info().key.as_ref()],
        ctx.program_id
    );
    let thread_id_vec: Vec<u8> = thread_id.to_bytes().to_vec();
    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: affair_account.affair_termination_time as u64,
    };

    // Step 6: Fetch the bump seed associated with the authority
    let (pda, bump) = Pubkey::find_program_address(&[SEED_AUTHORITY_THREAD], ctx.program_id);

    // Step 7: Create the termination thread
    let cpi_ctx = anchor_lang::context::CpiContext::new_with_signer(
        ctx.accounts.affair_clockwork_thread.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: ctx.accounts.creator.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            thread: ctx.accounts.affair_clockwork_thread.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        &[&[SEED_AUTHORITY_THREAD, &[bump]]],
    );

    // Execute the thread creation
    thread_create(
        cpi_ctx,
        LAMPORTS_PER_SOL,
        thread_id_vec,
        vec![terminate_affair_instruction.into()],
        trigger,
    )?;

    // Step 9: Add Affair to Affair List
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let affair_pubkey = *ctx.accounts.affair.to_account_info().key;

    // Register the affair
    affairs_list_account.register_affair(affair_pubkey)?;

    // Step 10: All steps successful, return Ok
    Ok(())
}

/*
#[derive(Accounts)]
pub struct InitializeThread<'info> {
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    #[account(mut, address = clockwork_sdk::state::Thread::pubkey(thread_authority.key(), active_rental.key().to_bytes().to_vec()))]
    pub thread: Account<'info, clockwork_sdk::state::Thread>,
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: Account<'info, clockwork_sdk::state::Thread>,
    pub active_rental: AccountInfo<'info>,
}
*/