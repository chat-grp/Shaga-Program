use anchor_lang::prelude::*;
use crate::{InitializeSession, Session, states::SessionState};


#[derive(anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]
pub struct SessionPayload {
    pub ip_address: [u8; 15],
    pub cpu_name: [u8; 64],  
    pub gpu_name: [u8; 64],  
    pub total_ram_mb: u32,
    pub usdc_per_hour: u32,
}

impl Default for SessionPayload {
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
    ctx: Context<InitializeSession>,
    payload: SessionPayload,
) -> Result<()> {
    let session_account = &mut ctx.accounts.session;
    let lender_account = &mut ctx.accounts.lender;

    let mut session_object = Session::default();

    session_object.lender = *ctx.accounts.creator.unsigned_key();
    session_object.ip_address = payload.ip_address;
    session_object.cpu_name = payload.cpu_name;
    session_object.gpu_name = payload.gpu_name;
    session_object.total_ram_mb = payload.total_ram_mb;
    session_object.usdc_per_hour = payload.usdc_per_hour;

    session_object.session_state = SessionState::Waiting;

    session_account.set_inner(session_object);

    Ok(())
}
