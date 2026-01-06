use anchor_lang::prelude::*;
// Musimy importować ResolvePoll, bo tak nazwaliśmy strukturę w programie Voting
use voting::{
    cpi::accounts::ResolvePoll, 
    cpi::resolve_poll,
    program::Voting,
};
use crate::Manager;

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [b"manager_seed", authority.key().as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    /// CHECK: Zmiana na UncheckedAccount czasami pomaga przy eskalacji uprawnień w CPI
    #[account(mut)] 
    pub poll: UncheckedAccount<'info>, 

    pub voting_program: Program<'info, Voting>,
    pub system_program: Program<'info, System>,
}

// Używamy anchor_lang::Result, aby uniknąć błędów z generykami
pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, ResolveEvent<'info>>, 
    poll_id: u64
) -> anchor_lang::Result<()> {
    
    // DEBUG: Sprawdźmy w logach, czy konta wchodzą jako writable
    for (i, acc) in ctx.remaining_accounts.iter().enumerate() {
        msg!("Remaining account {}: {} | writable: {}", i, acc.key, acc.is_writable);
    }
    // 1. Przygotowanie signera (Manager PDA)
    let authority_key = ctx.accounts.authority.key();
    let manager_seeds = &[
        b"manager_seed".as_ref(),
        authority_key.as_ref(),
        &[ctx.bumps.manager],
    ];
    let signer = &[&manager_seeds[..]];

    // 2. Przygotowanie kont do CPI
    let cpi_accounts = ResolvePoll {
        authority: ctx.accounts.manager.to_account_info(),
        poll: ctx.accounts.poll.to_account_info(),
        receiver: ctx.accounts.authority.to_account_info(),
    };

    // 3. Kontekst CPI - używamy ctx.remaining_accounts.to_vec()
    // Dzięki dopisaniu 'info w sygnaturze funkcji, Rust teraz wie, 
    // że te konta żyją wystarczająco długo.
let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.voting_program.to_account_info(),
        cpi_accounts,
        signer,
    ).with_remaining_accounts(ctx.remaining_accounts.to_vec());

    voting::cpi::resolve_poll(cpi_ctx, poll_id)?;
    Ok(())
}