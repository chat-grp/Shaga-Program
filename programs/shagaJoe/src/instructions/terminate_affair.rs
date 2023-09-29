use anchor_lang::prelude::*;
use crate::{Terminateaffair, affairState, errors::ShagaErrorCode};
use solana_program::clock::Clock;
use clockwork_sdk::state::{Thread, Trigger, TriggerContext};

#[error]
pub enum ErrorCode {
    #[msg("The affair cannot be terminated before the scheduled time.")]
    InvalidTerminationTime,
}

pub fn handler(ctx: Context<Terminateaffair>) -> Result<()> {
    // Reference to the affair account
    let affair_account = &mut ctx.accounts.affair;

    // Reference to the clockwork_thread account
    let clockwork_thread = &ctx.accounts.clockwork_thread;

    // Fetch the current time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    // Check if the affair can be terminated
    if current_time < affair_account.affair_termination_time {
        msg!("affair cannot be terminated before the scheduled time.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }

    // Validate that the trigger condition for the Clockwork Thread is met
    if let TriggerContext::Timestamp { started_at } = clockwork_thread.exec_context.unwrap().trigger_context {
        if current_time < started_at {
            msg!("Clockwork Thread has not reached the trigger timestamp yet.");
            return Err(ShagaErrorCode::InvalidTerminationTime.into());
        }
    } else {
        msg!("Invalid trigger context for Clockwork Thread.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }

    // Update the affair state to Finished
    affair_account.affair_state = affairState::Finished;

    Ok(())
}
