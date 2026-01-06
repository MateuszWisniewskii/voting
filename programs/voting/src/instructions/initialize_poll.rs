use anchor_lang::prelude::*;
use crate::{ANCHOR_DISCRIMINATOR_SIZE, Poll};

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // <--- DODAJ TO KONTO

    /// CHECK: To jest PDA Managera, który autoryzuje akcję
    pub authority: UncheckedAccount<'info>, // <--- ZMIEŃ Signer na UncheckedAccount

    #[account(
        init,
        payer = payer, // <--- ZMIEŃ z authority na payer
        space = ANCHOR_DISCRIMINATOR_SIZE + Poll::INIT_SPACE,
        seeds = [b"poll_seed".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializePoll>, 
    poll_id: u64, 
    start_time: u64, 
    end_time: u64, 
    poll_name: String, 
    poll_description: String
) -> Result<()> {
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