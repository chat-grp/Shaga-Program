use crate::{errors::*, seeds::*, states::*, ID};
use anchor_lang::prelude::*;
use clockwork_sdk::state::{Thread, TriggerContext};

use solana_program::{clock::Clock, system_instruction};

#[derive(Accounts)]
pub struct TerminateVacantAffairAccounts<'info> {
    #[account(mut)]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,

    /// Verify that only this thread can execute the ThreadTick Instruction
    #[account(signer, constraint = thread.authority.eq(&thread_authority.key()))]
    pub thread: Account<'info, Thread>,

    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    /// The Thread Admin
    /// The authority that was used as a seed to derive the thread address
    /// `thread_authority` should equal `thread.thread_authority`
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_vacant_affair_termination(ctx: Context<TerminateVacantAffairAccounts>) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let lender = &ctx.accounts.lender;
    let vault = &ctx.accounts.vault;
    let system_program = &ctx.accounts.system_program;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // handled by anchor
    affair_account.close(vault.to_account_info())?;

    Ok(())
}
