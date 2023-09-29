use anchor_lang::prelude::*;
use anchor_spl::token::Token;
//use session_keys::{SessionTargeted, SessionToken};

declare_id!("CtvfzWET3tWdfsDbyV6BDqLfSKDwYgtraPkdnAJw6UEt"); 

mod instructions;
mod states;
pub mod errors;
mod seeds;
mod checks;

use instructions::*;
use states::*;

#[program]
pub mod shaga {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }


    pub fn initialize_lender(ctx: Context<InitializeLender>) -> Result<()> {
        create_lender::handler(ctx)
    }


    pub fn initialize_session(ctx: Context<InitializeSession>, payload: SessionPayload) -> Result<()> {
        create_session::handler(ctx, payload)
    }

    pub fn join_session(ctx: Context<JoinSession>, rent_amount: u64, rental_termination_time: i64) -> Result<()> {
        join_session::handler(ctx, rent_amount, rental_termination_time)
    }

    pub fn terminate_session(ctx: Context<TerminateSession>, termination_by: TerminationAuthority) -> Result<()> {
        terminate_session::handler(ctx, termination_by)
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer=payer, space = SessionsList::size(), seeds=[seeds::SEED_SESSION_LIST], bump)]
    pub sessions_list: Account<'info, SessionsList>,

    #[account(init, payer=payer, space = Escrow::size(), seeds=[seeds::SEED_ESCROW], bump)]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeLender<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer=payer, space = Lender::size(), seeds=[seeds::SEED_LENDER, payer.key().as_ref()], bump)]
    pub lender: Account<'info, Lender>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeSession<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut, address=Lender::pda(creator.key()).0)]
    pub lender: Account<'info, Lender>,
    #[account(init, payer = creator, space = Session::size(), seeds = [seeds::SEED_SESSION, lender.key().as_ref()], bump)]
    pub session: Account<'info, Session>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct JoinSession<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut)]
    pub session: Account<'info, Session>,
    #[account(mut)]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TerminateSession<'info> {
    #[account(mut)]
    pub system: Signer<'info>,  // Placeholder, you may need to replace it with an actual system account
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut)]
    pub server: Signer<'info>,
    #[account(mut)]
    pub session: Account<'info, Session>,
    #[account(mut)]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    pub lender_token_account: AccountInfo<'info>,
    pub client_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}