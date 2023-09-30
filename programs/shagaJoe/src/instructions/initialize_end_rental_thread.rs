use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use clockwork_sdk::state::Trigger;
use crate::{InitializeThread, TerminationAuthority};
use clockwork_sdk::cpi::CpiContext;


pub fn initialize_end_rental_thread(ctx: Context<InitializeThread>, thread_id: Vec<u8>) -> Result<()> {

    // Step 1: Initialize EndRental data
    let termination_by = TerminationAuthority::Clockwork;

    // Step 2: Convert Accounts to AccountMeta
    let affair_meta = AccountMeta::new(*ctx.accounts.active_rental.unwrap(), false);  // Assuming active_rental holds the affair account
    // Add other necessary accounts here

    // Step 3: Create the Instruction
    let end_rental_instruction = Instruction {
        program_id: crate::ID,
        accounts: vec![affair_meta], // Add other accounts here
        data: termination_by.try_to_vec().unwrap(),
    };

    // Step 4: Define the Trigger
    let trigger = Trigger::Cron {
        schedule: "*/10 * * * * * *".to_string(),
        skippable: true,
    };

    // Step 5: Create Thread
    clockwork_sdk::cpi::thread_create(
        CpiContext::new_with_signer(
            ctx.accounts.clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadCreate {
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                thread: ctx.accounts.thread.to_account_info(),
                authority: ctx.accounts.thread_authority.to_account_info(),
            },
            &[], // Signers, if any
        ),
        LAMPORTS_PER_SOL,       // Amount, adjust as needed
        thread_id,              // Thread ID
        vec![end_rental_instruction.into()], // Instructions
        trigger,                // Trigger
    )?;

    Ok(())
}
