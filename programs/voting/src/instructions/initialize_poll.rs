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

pub fn handler(ctx: Context<InitializePoll>, poll_id: u64, start_time: u64, end_time: u64, poll_name: String, poll_description: String) -> Result<()> {
    let poll = &mut ctx.accounts.poll;

    poll.authority = ctx.accounts.authority.key();
    poll.poll_name = poll_name;
    poll.poll_description = poll_description;
    poll.start_time = start_time;
    poll.end_time = end_time;
    poll.poll_option_index = 0;
    poll.winning_candidate = "Jeszcze nie wyłoniono zwycięskiego kandydata".to_string();
    
    Ok(())
}
