use anchor_lang::prelude::*;
use crate::{InitializeLender, Lender, errors::ShagaErrorCode};


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