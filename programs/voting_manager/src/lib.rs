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

    pub fn create_event(ctx: Context<CreateEvent>) -> Result<()> {
        create_event::handler(ctx)
    }

    pub fn resolve_event(ctx: Context<ResolveEvent>) -> Result<()> {
        resolve_event::handler(ctx)
    }
}
