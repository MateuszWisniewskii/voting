use anchor_lang::prelude::*;

use crate::{ANCHOR_DISCRIMINATOR_SIZE, Poll};

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Poll::INIT_SPACE,
        seeds = [b"poll_seed".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializePoll>, poll_id: u64) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
