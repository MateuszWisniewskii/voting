pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4kQ7rpUTcxYfSLVWcfFudGuUhxPCwqsDZw1z33gEDbBf");

#[program]
pub mod voting_manager {
    use super::*;

    pub fn initialize_manager(ctx: Context<InitializeManager>) -> Result<()> {
        initialize_manager::handler(ctx)
    }

    pub fn create_event(
        ctx: Context<CreateEvent>,
        start_time: u64,
        end_time: u64,
        poll_name: String,
        poll_description: String,
        candidate_names: Vec<String>
    ) -> Result<()> {
        create_event::handler(ctx, start_time, end_time, poll_name, poll_description, candidate_names)
    }

    pub fn resolve_event(ctx: Context<ResolveEvent>) -> Result<()> {
        resolve_event::handler(ctx)
    }
}
