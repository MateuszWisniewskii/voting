pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3QtBbSDvHi2wAZe1akqUSbbWQ2VSN9iADkqsTgT6J5SR");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
    ctx: Context<InitializePoll>, // To Context używa teraz struktury z 'payer'
    poll_id: u64,
    start_time: u64,
    end_time: u64,
    poll_name: String,
    poll_description: String,
) -> Result<()> {
    // Przekazujemy ctx dalej do handlera
    instructions::initialize_poll::handler(ctx, poll_id, start_time, end_time, poll_name, poll_description)
}

pub fn initialize_candidate(
    ctx: Context<InitializeCandidate>, // Tutaj też struktura ma już 'payer'
    poll_id: u64,
    candidate_name: String,
) -> Result<()> {
    instructions::initialize_candidate::handler(ctx, poll_id, candidate_name)
}

    pub fn vote(ctx: Context<Vote>, poll_id: u64, candidate_name: String) -> Result<()> {
        vote::handler(ctx, poll_id, candidate_name)
    }

    // pub fn voting_status(ctx: Context<VotingStatus>, poll_id: u64) -> Result<()> {
    //     voting_status(ctx, poll_id)
    // }
}
