use crate::{errors::*, seeds::*, states::*};
use anchor_lang::prelude::*;
use clockwork_sdk::state::Thread;

#[derive(Accounts)]
pub struct TerminateVacantAffairAccounts<'info> {
    /// checked below if signer == client or thread
    #[account(mut)]
    pub signer: Signer<'info>,
    // /// Verify that only this thread can execute the ThreadTick Instruction
    // #[account(signer, constraint = thread.authority.eq(&thread_authority.key()))]
    // pub thread: Account<'info, Thread>,
    #[account(mut, seeds = [SEED_AFFAIR, affair.authority.as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    /// The Thread Admin
    /// The authority that was used as a seed to derive the thread address
    /// `thread_authority` should equal `thread.thread_authority`
    /// CHECK: via seeds
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_vacant_affair_termination(ctx: Context<TerminateVacantAffairAccounts>) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let vault = &ctx.accounts.vault;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let signer = &ctx.accounts.signer;
    let thread_authority = &ctx.accounts.thread_authority;

    // check if signer is the client
    if affair_account.authority != signer.key() {
        // check if signer is thread. if it is not then fail early.
        // serialize the signer into a thread or fail.
        let thread_data = &mut &**signer.try_borrow_data()?;
        let thread_signer_result = Thread::try_deserialize(thread_data);
        let thread_signer = if thread_signer_result.is_ok() {
            thread_signer_result.unwrap()
        } else {
            msg!("Could not deserialize clockwork thread termination key.");
            return Err(ShagaErrorCode::InvalidSigner.into());
        };
        if !thread_signer.authority.eq(&thread_authority.key()) {
            msg!("Invalid clockwork thread rental termination key.");
            return Err(ShagaErrorCode::InvalidSigner.into());
        }
    }
    if affair_account.rental.is_some() {
        msg!("Invalid instruction there is an on going rental.");
        return Err(ShagaErrorCode::InvalidTerminationInstruction.into());
    }

    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    affairs_list_account.remove_affair(affair_pubkey);

    // handled by anchor
    affair_account.close(vault.to_account_info())?;

    Ok(())
}
