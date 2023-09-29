use anchor_lang::prelude::*;
use crate::{Initializeaffair, states::affair, states::affairState};


#[derive(anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]
pub struct affairPayload {
    pub ip_address: [u8; 15],
    pub cpu_name: [u8; 64],  
    pub gpu_name: [u8; 64],  
    pub total_ram_mb: u32,
    pub usdc_per_hour: u32,
}

impl Default for affairPayload {
    fn default() -> Self {
        Self {
            ip_address: [0u8; 15],
            cpu_name: [0u8; 64],
            gpu_name: [0u8; 64],
            total_ram_mb: 0,
            usdc_per_hour: 0,
        }
    }
}


pub fn handler(
    ctx: Context<Initializeaffair>,
    payload: affairPayload,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let lender_account = &mut ctx.accounts.lender;

    let mut affair_object = affair::default();

    affair_object.lender = *ctx.accounts.creator.unsigned_key();
    affair_object.ip_address = payload.ip_address;
    affair_object.cpu_name = payload.cpu_name;
    affair_object.gpu_name = payload.gpu_name;
    affair_object.total_ram_mb = payload.total_ram_mb;
    affair_object.usdc_per_hour = payload.usdc_per_hour;

    affair_object.affair_state = affairState::Waiting;

    affair_account.set_inner(affair_object);

    Ok(())
}
