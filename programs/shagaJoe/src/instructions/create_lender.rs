use crate::seeds::SEED_LENDER;
use crate::{errors::ShagaErrorCode, Lender};
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<InitializeLender>) -> Result<()> {
    let lender_account = &mut ctx.accounts.lender;

    if lender_account.authority != Pubkey::default() {
        return Err(ShagaErrorCode::InvalidLender.into());
    }

    let lender_object = Lender::default();
    lender_account.set_inner(lender_object);
    lender_account.authority = *ctx.accounts.payer.unsigned_key();

    Ok(())
}

// impl<'info> InitializeLender<'info> {
//     pub fn is_authorized_to_init_lender(creator: &AccountInfo) -> Result<()> {
//         Ok(())
//     }
// }

#[derive(Accounts)]
pub struct InitializeLender<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = Lender::size(), seeds = [SEED_LENDER], bump)]
    pub lender: Account<'info, Lender>,
    pub system_program: Program<'info, System>,
}
