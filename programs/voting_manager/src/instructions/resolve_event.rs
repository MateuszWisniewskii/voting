use anchor_lang::prelude::*;

use voting::{
    cpi::{
        
    }
};

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ResolveEvent>) -> Result<()> {

    Ok(())
}