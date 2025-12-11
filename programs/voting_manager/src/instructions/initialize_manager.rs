use anchor_lang::prelude::*;
use crate::ANCHOR_DISCRIMINATOR_SIZE;

use crate::Manager;

#[derive(Accounts)]
pub struct InitializeManager<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Manager::INIT_SPACE,
        seeds = [b"manager_seed".as_ref(), authority.key().as_ref()],
        bump
    )]
    pub manager: Account<'info, Manager>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeManager>) -> Result<()> {
    let manager = &mut ctx.accounts.manager;

    manager.authority = ctx.accounts.authority.key();
    manager.poll_count = 0;
    
    Ok(())
}