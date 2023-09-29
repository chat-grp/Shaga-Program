use anchor_lang::prelude::*;
use crate::seeds::SEED_ESCROW;


#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Escrow {
    pub locked_amount: u64,
}

impl Escrow {
    pub fn size() -> usize {
        8 + Escrow::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_ESCROW], &crate::ID)
    }
}