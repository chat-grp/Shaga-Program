use anchor_lang::prelude::*;
use clockwork_sdk::state::{ ThreadAccount};


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

    pub fn initialize(ctx: Context<Initialize>
    ) -> Result<()> {
        Ok(())
    }


    pub fn initialize_lender(ctx: Context<InitializeLender>
    ) -> Result<()> {
        create_lender::handler(ctx)
    }


    pub fn create_affair(ctx: Context<AffairAccounts>,
                         payload: AffairPayload
    ) -> Result<()> {
        create_affair::handler(ctx, payload)
    }

    pub fn start_rental(ctx: Context<RentalAccounts>,
                        rental_termination_time: i64
    ) -> Result<()> {
        start_rental::handler(ctx, rental_termination_time)
    }

    pub fn end_rental(ctx: Context<RentalAccounts>,
                      termination_by: RentalTerminationAuthority
    ) -> Result<()> {
        end_rental::handler(ctx, termination_by)
    }

    pub fn terminate_affair(
        ctx: Context<AffairAccounts>,
        termination_by: AffairTerminationAuthority
    ) -> Result<()> {
        terminate_affair::handler(ctx, termination_by)
    }

    /*
    pub fn collect_fees{
        collectale
    }
    */
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = AffairsList::size(), seeds = SEED_AFFAIR_LIST, bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(init, payer=payer, space = Escrow::size(), seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}




#[derive(Accounts)]
pub struct AffairAccounts<'info> {
    #[account(mut, constraint = is_authorized_to_init_affair(creator))]
    pub creator: Signer<'info>,
    #[account(mut, address=Lender::pda(creator.key()).0)]
    pub lender: Account<'info, Lender>,
    #[account(init, payer = creator, space = affair::size(), seeds = SEED_affair, bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    #[account(has_one = authority)]
    pub clockwork_thread: Account<'info, clockwork_sdk::state::Thread>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RentalAccounts<'info> {
    #[account(mut, signer)]
    pub client: AccountInfo<'info>,
    #[account(mut)]
    pub lender: AccountInfo<'info>,
    #[account(mut)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(init, payer = client, space = Escrow::size(), seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = client, space = Rental::size(), seeds = SEED_RENTAL, bump)]
    pub rental: Account<'info, Rental>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    #[account(signer)]
    pub clockwork_thread: AccountInfo<'info>,
}


/* TODO: OLD VERSION WITH CUSTOM TOKEN PROGRAM (WE'RE USING LAMPORTS IN MVP)
#[derive(Accounts)]
pub struct RentalAccounts<'info> {
    #[account(mut, signer)]
    pub client: AccountInfo<'info>,
    #[account(mut)]
    pub client_token_account: Account<'info, anchor_spl::token::TokenAccount>,
    #[account(mut)]
    pub lender_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub affair: Account<'info, Affair>,
    #[account(mut)]
    pub lender: Account<'info, Lender>,
    #[account(init, payer = client, space = Escrow::size(), seeds = [SEED_ESCROW, affair.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = client, space = Rental::size(), seeds = SEED_RENTAL, bump)]
    pub rental: Account<'info, Rental>,
    #[account(seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    #[account(signer)]
    pub clockwork_thread: AccountInfo<'info>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    pub system_program: Program<'info, System>,
}
 */