use anchor_lang::prelude::*;

use crate::{error::VotingError, Candidate, Poll, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll_seed".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init_if_needed,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Candidate::INIT_SPACE,
        seeds = [b"candidate_seed".as_ref(), poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeCandidate>,
    poll_id: u64,
    candidate_name: String,
) -> Result<()> {
    let candidate = &mut ctx.accounts.candidate;
    let poll = &mut ctx.accounts.poll;
    let current_time = Clock::get()?.unix_timestamp;

    if current_time > (poll.start_time as i64) {
        return Err(VotingError::AddingCandidateAfterVotingStart.into());
    }

    candidate.candidate_name = candidate_name;
    candidate.candidate_votes = 0;
    poll.poll_option_index = poll.poll_option_index.checked_add(1).ok_or(VotingError::Overflow)?;

    Ok(())
}
