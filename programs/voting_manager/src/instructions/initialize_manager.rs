use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeManager<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeManager>) -> Result<()> {

    Ok(())
}