use anchor_lang::prelude::*;

use crate::{ANCHOR_DISCRIMINATOR_SIZE, Candidate};

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Candidate::INIT_SPACE,
        seeds = [b"candidate_seed".as_ref(), poll_id.to_be_bytes().as_ref(), candidate_name.as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeCandidate>, poll_id: u64, candidate_name: String) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
