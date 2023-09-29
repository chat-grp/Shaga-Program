use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use clockwork_sdk::state::Thread;
use session_keys::{Session, SessionToken};

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


    pub fn initialize_affair(ctx: Context<InitializeAffair>, payload: affairPayload) -> Result<()> {
        create_affair::handler(ctx, payload)
    }

    pub fn start_rental(ctx: Context<StartRental>, rental_termination_time: i64) -> Result<()> {
        start_rental::handler(ctx, rental_termination_time)
    }

    pub fn end_rental(ctx: Context<EndRental>, termination_by: TerminationAuthority) -> Result<()> {
        end_rental::handler(ctx, termination_by)
    }

    pub fn terminate_affair(ctx: Context<TerminateAffair>, termination_by: TerminationAuthority) -> Result<()> {
        terminate_affair::handler(ctx, termination_by)
    }
}




#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = affairsList::size(), seeds = SEED_affair_LIST, bump)]
    pub affairs_list: Account<'info, AffairsList>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct InitializeLender<'info> {
    #[account(mut, constraint = is_authorized_to_init_lender(payer))]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = Lender::size(), seeds = SEED_LENDER, bump)]
    pub lender: Account<'info, Lender>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAffair<'info> {
    #[account(mut, constraint = is_authorized_to_init_affair(creator))]
    pub creator: Signer<'info>,
    #[account(mut, address=Lender::pda(creator.key()).0)]
    pub lender: Account<'info, Lender>,
    #[account(init, payer = creator, space = affair::size(), seeds = SEED_affair, bump)]
    pub affair: Account<'info, affair>,
    pub system_program: Program<'info, System>,
    pub clockwork_thread: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TerminateAffair<'info> {
    #[account(mut, constraint = is_authorized_to_terminate(affair, authority))]
    pub affair: Account<'info, Affair>,
    pub system_program: Program<'info, System>,
    #[account(has_one = authority)]
    pub clockwork_thread: Account<'info, Thread>,
    pub authority: Signer<'info>,
}


#[derive(Accounts)]
pub struct StartRental<'info> {
    #[account(mut, signer)]
    pub client: AccountInfo<'info>,
    #[account(mut)]
    pub affair: Account<'info, affair>,
    #[account(init, payer = client, space = Escrow::size(), seeds = SEED_ESCROW, bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = client, space = Rental::size(), seeds = SEED_RENTAL, bump)]
    pub rental: Account<'info, Rental>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndRental<'info> {
    pub rental: Account<'info, Rental>,
    pub clockwork_thread: AccountInfo<'info>,
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut)]
    pub affair: Account<'info, affair>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    pub lender_token_account: AccountInfo<'info>,
    pub client_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}