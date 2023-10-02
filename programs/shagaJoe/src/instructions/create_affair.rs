use crate::{errors::*, seeds::*, states::*, ID};
use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use clockwork_sdk::cpi::thread_create;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
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

#[derive(Accounts)]
pub struct CreateAffairAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, address=Lender::pda(authority.key()).0)]
    pub lender: Account<'info, Lender>,
    #[account(init, payer = authority, space = Affair::INIT_SPACE, seeds = [SEED_AFFAIR], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,
    /// CHECK: checked below
    #[account(mut)]
    pub affair_clockwork_thread: UncheckedAccount<'info>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    /// CHECK: checked below
    /// The pda that will own and manage the thread.
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

/// creates an affair by the lender/pc owner/creator.
pub fn handle_create_affair(
    ctx: Context<CreateAffairAccounts>,
    payload: AffairPayload,
) -> Result<()> {
    // Step 1: Initialize mutable references for Affair and Lender accounts
    let affair_account = &mut ctx.accounts.affair;
    let authority = &ctx.accounts.authority;
    let lender = &ctx.accounts.lender;
    let system_program = &ctx.accounts.system_program;
    let affair_clockwork_thread = &ctx.accounts.affair_clockwork_thread;
    let thread_authority = &ctx.accounts.thread_authority;
    let authority = &ctx.accounts.authority;
    let vault = &ctx.accounts.vault;
    let affairs_list_account = &mut ctx.accounts.affairs_list;

    // Step 2A: Populate it with payload and default values
    affair_account.authority = authority.key();
    affair_account.ip_address = payload.ip_address;
    affair_account.cpu_name = payload.cpu_name;
    affair_account.gpu_name = payload.gpu_name;
    affair_account.total_ram_mb = payload.total_ram_mb;
    affair_account.usdc_per_hour = payload.usdc_per_hour;
    affair_account.affair_termination_time = payload.affair_termination_time as u64;
    affair_account.affair_state = AffairState::Available;

    let borrow_affair_account = affair_account.clone();
    // Step 2B: Accounts for terminate_affair instruction
    // Step 2C: Create the terminate_affair_instruction
    let target_ix = Instruction {
        program_id: ID,
        accounts: crate::__client_accounts_terminate_vacant_affair_accounts::TerminateVacantAffairAccounts {
          thread: affair_clockwork_thread.key(),
          thread_authority: thread_authority.key(),
            lender: lender.key(),
            affair: affair_account.key(),
            affairs_list: affairs_list_account.key(),
            vault: vault.key(),
            system_program: system_program.key(),
        }
        .to_account_metas(Some(true)),
        data: crate::instruction::TerminateVacantAffair {}.data(),
    };

    // Step 3: Fetch the current timestamp and validate affair termination time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    if affair_account.affair_termination_time <= current_time {
        msg!("Affair termination time must be in the future.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }
    // , address = Thread::pubkey(thread_authority.key(), thread_id)
    // Step 5: Define thread_id with seeds & trigger for the termination thread
    let (thread_id, _bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.key().as_ref(),
            borrow_affair_account.key().as_ref(),
        ],
        ctx.program_id,
    );
    let thread_id_vec: Vec<u8> = thread_id.to_bytes().to_vec();
    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: affair_account.affair_termination_time as i64,
    };

    // Step 6: Fetch the bump seed associated with the authority
    let (clockwork_thread_computed, _bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.key().as_ref(),
            borrow_affair_account.key().as_ref(),
        ],
        ctx.program_id,
    );
    if clockwork_thread_computed.key() != affair_clockwork_thread.key() {
        msg!("Invalid clockwork thread affair termination key.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }
    let bump = *ctx.bumps.get("thread_authority").unwrap();
    let cpi_signer: &[&[u8]] = &[SEED_AUTHORITY_THREAD, &[bump]];
    let binding_seeds = &[cpi_signer];
    // Step 7: Create the termination thread
    let cpi_ctx = CpiContext::new_with_signer(
        affair_clockwork_thread.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: authority.to_account_info(),
            system_program: system_program.to_account_info(),
            thread: affair_clockwork_thread.to_account_info(),
            authority: thread_authority.to_account_info(),
        },
        binding_seeds,
    );

    // Execute the thread creation
    thread_create(
        cpi_ctx,
        LAMPORTS_PER_SOL,
        thread_id_vec,
        vec![target_ix.into()],
        trigger,
    )?;

    // Step 9: Add Affair to Affair List
    let affair_pubkey = affair_account.key();

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
