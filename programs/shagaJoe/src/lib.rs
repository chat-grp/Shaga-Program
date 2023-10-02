use anchor_lang::prelude::*;
pub mod checks;
pub mod errors;
pub mod instructions;
pub mod seeds;
pub mod states;
pub use {checks::*, errors::*, instructions::*, seeds::*, states::*};

declare_id!("6AACcBoHBKc2XndsuQpgf6S9M5HP8jDUsgbn7R6EJAMW");
#[program]
pub mod shaga {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_lender(ctx: Context<InitializeLender>) -> Result<()> {
        create_lender::handle_lender_initialization(ctx)
    }

    pub fn create_affair(ctx: Context<CreateAffairAccounts>, payload: AffairPayload) -> Result<()> {
        create_affair::handle_create_affair(ctx, payload)
    }

    pub fn start_rental(
        ctx: Context<StartRentalAccounts>,
        rental_termination_time: u64,
    ) -> Result<()> {
        start_rental::handle_starting_rental(ctx, rental_termination_time)
    }

    pub fn end_rental(
        ctx: Context<EndRentalAccounts>,
        termination_by: RentalTerminationAuthority,
    ) -> Result<()> {
        end_rental::handle_ending_rental(ctx, termination_by)
    }

    pub fn terminate_affair(ctx: Context<TerminateAffairAccounts>) -> Result<()> {
        terminate_affair::handle_affair_termination(ctx)
    }
    /// handled by clockwork
    pub fn terminate_vacant_affair(ctx: Context<TerminateVacantAffairAccounts>) -> Result<()> {
        terminate_vacant_affair::handle_vacant_affair_termination(ctx)
    }

    /* TODO: filter who can init affairs
    pub fn is_authorized_to_init_affair(creator: &AccountInfo) -> Result<()> {
        let client_pubkey = creator.key;

        let lender_data: Lender = Lender::try_from_slice(&creator.data.borrow())?;
        if &lender_data.authority == client_pubkey {
            Ok(())
        } else {
            msg!("Only registered lenders can start affairs"); // karma > BAN_VALUE
            return Err(ErrorCode::ShagaErrorCode::UnauthorizedAffairCreation.into());
        }
    }
    */

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
    #[account(init, payer=payer, space = AffairsList::size(), seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(init, payer=payer, space = Escrow::INIT_SPACE, seeds = [SEED_ESCROW], bump)]
    pub vault: Account<'info, Escrow>,
    /// The pda that will own and manage threads.
    #[account(init, payer=payer, space = Escrow::INIT_SPACE, seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
