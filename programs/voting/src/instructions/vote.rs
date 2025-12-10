use anchor_lang::prelude::*;

use crate::{error::VotingError, Candidate, ParticipantVote, Poll, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll_seed".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [b"candidate_seed".as_ref(), poll_id.to_be_bytes().as_ref(), candidate_name.as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        init_if_needed,
        payer = participant,
        space = ANCHOR_DISCRIMINATOR_SIZE + ParticipantVote::INIT_SPACE,
        seeds = [b"vote_seed".as_ref(), poll_id.to_le_bytes().as_ref(), participant.key().as_ref()],
        bump
    )]
    pub participant_vote: Account<'info, ParticipantVote>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Vote>, poll_id: u64, candidate_name: String) -> Result<()> {
    let poll = &ctx.accounts.poll; // Nie mutowalne ze ponieważ tylko sprawdzamy czas rozpoczęcia/zakończenia głosowania
    let candidate = &mut ctx.accounts.candidate;
    let participant_vote = &mut ctx.accounts.participant_vote;
    let current_time = Clock::get()?.unix_timestamp;

    // Sprawdzamy czy uczestnik oddał już głos
    if participant_vote.vote_placed {
        return Err(VotingError::VoteHaveBeenPlaced.into());
    }

    // Sprawdzamy czy głos nie jest oddany po zakończeniu głosowania
    if current_time > (poll.start_time as i64) {
        return Err(VotingError::VotingEnded.into());
    }

    // Sprawdzamy czy głos nie jest offany przed rozpoczęciem głosowania
    if current_time <= (poll.start_time as i64) {
        return Err(VotingError::VotingNotStarted.into());
    }

    candidate.candidate_votes = candidate
        .candidate_votes
        .checked_add(1)
        .ok_or(VotingError::Overflow)?;

    participant_vote.participant = ctx.accounts.participant.key();
    participant_vote.poll_id = poll_id;
    participant_vote.candidate_name = candidate_name;
    participant_vote.vote_placed = true;
    
    Ok(())
}
