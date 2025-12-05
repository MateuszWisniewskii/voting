use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct VotingStatus<'info> {
    #[account(mut)]
    pub voting_participant: Signer<'info>,
    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VotingStatus>, poll_id: u64) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
