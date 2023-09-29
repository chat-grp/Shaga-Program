use anchor_lang::prelude::*;
use crate::seeds::SEED_ESCROW;


#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Escrow {
    pub locked_amount: u64,  // Add this field
}

impl Escrow {
    pub fn size() -> usize {
        8 + Escrow::INIT_SPACE // 8 bytes for the account header
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_ESCROW], &crate::ID)
    }
}