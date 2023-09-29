use anchor_lang::prelude::*;
use crate::seeds::SEED_AFFAIR_LIST;
pub const MAX_AFFAIR_S: usize = 10;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct AffairsList {
    #[max_len(10)]
    pub active_affairs: Vec<Pubkey>,
}

impl AffairsList {
    pub fn size() -> usize {
        8 + AffairsList::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_AFFAIR_LIST], &crate::ID)
    }

    pub fn register_affair(&mut self, affair: Pubkey) {
        // Remove the first affair if the list is full
        if self.active_affairs.len() >= MAX_AFFAIR_S {
            self.active_affairs.drain(..1);
        }
        // Add the new affair
        self.active_affairs.push(affair);
    }
}
