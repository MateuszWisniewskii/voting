use anchor_lang::prelude::*;

use voting::{
    cpi::{
        initialize_poll,
        initialize_cadidate
    }
};

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateEvent>) -> Result<()> {

    Ok(())
}