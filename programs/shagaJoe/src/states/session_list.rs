use anchor_lang::prelude::*;
use crate::seeds::SEED_SESSION_LIST; // Make sure SEED_SESSION_LIST is defined in seeds.rs

pub const MAX_SESSIONS: usize = 10;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct SessionsList {
    #[max_len(10)]
    pub active_sessions: Vec<Pubkey>,
}

impl SessionsList {
    pub fn size() -> usize {
        8 + SessionsList::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_SESSION_LIST], &crate::ID)
    }

    pub fn register_session(&mut self, session: Pubkey) {
        // Remove the first session if the list is full
        if self.active_sessions.len() >= MAX_SESSIONS {
            self.active_sessions.drain(..1);
        }
        // Add the new session
        self.active_sessions.push(session);
    }
}
