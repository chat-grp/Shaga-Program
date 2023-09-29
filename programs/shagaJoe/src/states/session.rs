// states/session.rs

use anchor_lang::prelude::*;


#[derive(InitSpace, Debug, anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SessionState {
    Waiting,
    Active,
    Finished { client: Pubkey },
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState::Waiting
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Session {
    pub lender: Pubkey,
    pub client: Pubkey,
    pub ip_address: [u8; 15],
    pub cpu_name: [u8; 64],
    pub gpu_name: [u8; 64],
    pub total_ram_mb: u32,
    pub usdc_per_hour: u32,
    pub session_state: SessionState,
    pub rental_start_time: i64,
    pub session_termination_time: i64,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            lender: Pubkey::default(),
            client: Pubkey::default(),
            ip_address: [0u8; 15],  // Default value
            cpu_name: [0u8; 64],    // Default value
            gpu_name: [0u8; 64],    // Default value
            total_ram_mb: 0,        // Default value
            usdc_per_hour: 0,       // Default value
            session_state: SessionState::default(),
            rental_start_time: 0,
            session_termination_time: 0,
        }
    }
}


impl Session {

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[crate::seeds::SEED_SESSION, owner.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Session::INIT_SPACE
    }

    pub fn is_session_active(&self) -> bool {
        self.session_state == SessionState::Waiting || self.session_state == SessionState::Active
    }

    pub fn can_settle(&self, client: &Pubkey) -> bool {
        match self.session_state {
            SessionState::Finished { client: ref c } => *c == *client,
            _ => false,
        }
    }

     pub fn can_join(&self) -> bool {
        // A session can be joined only if it is in the Waiting state
        self.session_state == SessionState::Waiting
    }

    pub fn join(&mut self, client_pubkey: Pubkey) {
        // Set the client's pubkey
        self.client = client_pubkey;
        
        // Update the session state to Active
        self.session_state = SessionState::Active;
    }
}