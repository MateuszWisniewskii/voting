pub mod constants;
pub mod error;
pub mod instructions; // To zakłada, że masz folder src/instructions/
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;
// Kluczowe: wyciągamy wszystko z modułu instructions, aby #[program] to widział
pub use instructions::*; 

declare_id!("3QtBbSDvHi2wAZe1akqUSbbWQ2VSN9iADkqsTgT6J5SR");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        start_time: u64,
        end_time: u64,
        poll_name: String,
        poll_description: String,
    ) -> Result<()> {
        instructions::initialize_poll::handler(ctx, poll_id, start_time, end_time, poll_name, poll_description)
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        poll_id: u64,
        candidate_name: String,
    ) -> Result<()> {
        instructions::initialize_candidate::handler(ctx, poll_id, candidate_name)
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, candidate_name: String) -> Result<()> {
        instructions::vote::handler(ctx, poll_id, candidate_name)
    }

    pub fn resolve_poll(ctx: Context<ResolvePoll>, poll_id: u64) -> Result<()> {
        instructions::resolve_poll::handler(ctx, poll_id)
    }
}