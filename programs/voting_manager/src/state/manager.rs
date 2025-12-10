use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Manager {
    pub authority: Pubkey,
    pub poll_count: u64,
}