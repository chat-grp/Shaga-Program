// states/affair.rs

use anchor_lang::prelude::*;
use crate::seeds::SEED_AFFAIR;
use crate::errors::ShagaErrorCode;

#[derive(InitSpace, Debug, anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AffairState {
    Unavailable,
    Available,
}

impl Default for AffairState {
    fn default() -> Self {
        AffairState::Available
    }
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Affair {
    pub lender: Pubkey,
    pub rental: Option<Pubkey>,
    pub ip_address: [u8; 15],
    pub cpu_name: [u8; 64],
    pub gpu_name: [u8; 64],
    pub total_ram_mb: u32,
    pub usdc_per_hour: u32,
    pub affair_state: AffairState,
    pub affair_termination_time: u64,
    pub active_rental_start_time: u64,
    pub due_rent_amount: u64,
    //pub active_locked_amount: u64,
}

impl Default for Affair {
    fn default() -> Self {
        Self {
            lender: Pubkey::default(),
            rental: Option::from(Pubkey::default()),
            ip_address: [0u8; 15],
            cpu_name: [0u8; 64],
            gpu_name: [0u8; 64],
            total_ram_mb: 0,
            usdc_per_hour: 0,
            affair_state: AffairState::default(),
            affair_termination_time: 0,
            active_rental_start_time: 0,
            due_rent_amount: 0,
            //active_locked_amount: 0,
        }
    }
}

impl Affair {

    pub fn join(&mut self, rental_key: Pubkey) -> Result<()> {
        if self.affair_state != AffairState::Available {
            msg!("Affair is not available for joining.");
            return Err(ShagaErrorCode::AffairAlreadyJoined.into());
        }

        self.rental = Some(rental_key);
        self.affair_state = AffairState::Unavailable;
        Ok(())
    }

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_AFFAIR, owner.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Affair::INIT_SPACE
    }

    pub fn can_join(&self) -> bool {
        self.affair_state == AffairState::Available
    }
}