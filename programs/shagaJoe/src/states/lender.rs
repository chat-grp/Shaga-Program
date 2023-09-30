// states/lender.rs

use anchor_lang::prelude::*;
use crate::seeds::SEED_LENDER;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Lender {
    pub authority: Pubkey,
    pub affairs: u32,
    pub karma: i32,  // Using i32 to allow both positive and negative karma
}

impl Lender {
    pub fn size() -> usize {
        8 + Lender::INIT_SPACE
    }

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_LENDER, owner.as_ref()], &crate::ID)
    }

    pub fn increment_affairs(&mut self) {
        self.affairs += 1;
    }

     pub fn give_thumbs_up(&mut self) {
        self.karma += 1;
    }

    pub fn give_thumbs_down(&mut self) {
        self.karma -= 1;
    }
}