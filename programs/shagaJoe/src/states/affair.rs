// states/affair.rs

use anchor_lang::prelude::*;
use crate::seeds::SEED_AFFAIR;


#[derive(InitSpace, Debug, anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AffairState {
    Waiting,
    Active,
    Finished { client: Pubkey },
}

impl Default for AffairState {
    fn default() -> Self {
        AffairState::Waiting
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Affair {
    pub lender: Pubkey,
    pub ip_address: [u8; 15],
    pub cpu_name: [u8; 64],
    pub gpu_name: [u8; 64],
    pub total_ram_mb: u32,
    pub usdc_per_hour: u32,
    pub affair_state: AffairState,
    pub affair_termination_time: i64,
}

impl Default for Affair {
    fn default() -> Self {
        Self {
            lender: Pubkey::default(),
            ip_address: [0u8; 15],
            cpu_name: [0u8; 64],
            gpu_name: [0u8; 64],
            total_ram_mb: 0,
            usdc_per_hour: 0,
            affair_state: AffairState::default(),
            affair_termination_time: 0,
        }
    }
}


impl Affair {

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_AFFAIR, owner.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Affair::INIT_SPACE
    }

    pub fn is_affair_active(&self) -> bool {
        self.affair_state == AffairState::Waiting || self.affair_state == AffairState::Active
    }

    pub fn can_settle(&self, client: &Pubkey) -> bool {
        match self.affair_state {
            AffairState::Finished { client: ref c } => *c == *client,
            _ => false,
        }
    }

     pub fn can_join(&self) -> bool {
        // A affair can be joined only if it is in the Waiting state
        self.affair_state == AffairState::Waiting
    }

    pub fn join(&mut self, client_pubkey: Pubkey) {
        // Set the client's pubkey
        self.client = client_pubkey;
        
        // Update the affair state to Active
        self.affair_state = AffairState::Active;
    }
}