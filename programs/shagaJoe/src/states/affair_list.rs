use anchor_lang::prelude::*;
use crate::errors::ShagaErrorCode;
use crate::seeds::SEED_AFFAIR_LIST;
pub const MAX_AFFAIRS: usize = 100;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct AffairsList {
    pub active_affairs: [Pubkey; MAX_AFFAIRS],
}

impl AffairsList {
    pub fn size() -> usize {
        8 + AffairsList::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_AFFAIR_LIST], &crate::ID)
    }

    pub fn register_affair(&mut self, affair: Pubkey) -> Result<()> {
        // Reject if the list is already full
        if self.active_affairs.len() >= MAX_AFFAIRS {
            msg!("The list of active affairs is full. Cannot add a new affair.");
            return Err(ShagaErrorCode::AffairListFull.into());
        }

        // Add the new affair
        self.active_affairs.push(affair);
        Ok(())
    }
    pub fn remove_affair(&mut self, affair_to_remove: Pubkey) {
        self.active_affairs.retain(|&affair| affair != affair_to_remove);
    }
}