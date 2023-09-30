// states/rental.rs

use anchor_lang::prelude::*;
use crate::seeds::SEED_RENTAL;
// TODO: make it possible to extend the rental, if the affair_termination_time allows it
#[account]
#[derive(InitSpace, Debug)]
pub struct Rental {
    pub client: Pubkey,
    pub affair: Pubkey,
    pub rent_amount: u64,
    pub rental_start_time: u64,
    pub rental_termination_time: u64,
    pub rental_clockwork_thread_id: Pubkey,
}

impl Default for Rental {
    fn default() -> Self {
        Self {
            client: Pubkey::default(),
            affair: Pubkey::default(),
            rent_amount: 0,
            rental_start_time: 0,
            rental_termination_time: 0,
            rental_clockwork_thread_id: Pubkey::default(),
        }
    }
}

impl Rental {
    pub fn initialize(&mut self, client: Pubkey, affair: Pubkey, rent_amount: u64, rental_start_time: i64, rental_termination_time: i64, rental_clockwork_thread_id: Pubkey) {
        self.client = client;
        self.affair = affair;
        self.rent_amount = rent_amount;
        self.rental_start_time = rental_start_time as u64;
        self.rental_termination_time = rental_termination_time as u64;
        self.rental_clockwork_thread_id = rental_clockwork_thread_id;
    }

    pub fn pda(affair: Pubkey, client: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_RENTAL, affair.as_ref(), client.as_ref()], &crate::ID)
    }

    pub fn size() -> usize {
        8 + Rental::INIT_SPACE
    }
}