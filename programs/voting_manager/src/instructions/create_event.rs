use anchor_lang::prelude::*;

use voting::{
    cpi::{
        accounts::{
            InitializeCandidate as CpiInitializeCandidate, InitializePoll as CpiInitializePoll,
        },
        initialize_cadidate, initialize_poll,
    },
    program::Voting,
};

use crate::Manager;

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"manager_seed".as_ref(), authority.key().as_ref()],
        bump
    )]
    pub manager: Account<'info, Manager>,

    /// CHECK: To konto zostanie zainicjalizowane przez CPI w programie Voting.
    /// Musimy je tu przekazać jako UncheckedAccount (lub AccountInfo), bo ten program nie ma definicji Poll.
    /// Klient JS musi wyliczyć poprawny adres PDA [b"poll_seed", poll_id]
    #[account(mut)]
    pub poll: UncheckedAccount<'info>,

    #[account()]
    pub voting_program: Program<'info, Voting>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateEvent>,
    start_time: u64,
    end_time: u64,
    poll_name: String,
    poll_description: String,
    candidate_names: Vec<String>
) -> Result<()> {
    let manager = &mut ctx.accounts.manager;
    let poll_id = manager.poll_count; // Pobieramy aktualne ID
    let poll = &ctx.accounts.poll;
    let system_program = &ctx.accounts.system_program;
    let voting_progeam = &ctx.accounts.voting_program;

    // Manager musi podpisać CPI, bo w programie 'voting' authority musi być Signerem.
    // Używamy seeds managera.
    let authority_key = &ctx.accounts.authority.key();
    let manager_seeds = &[
        b"manager_seed".as_ref(),
        authority_key.as_ref(),
        &[ctx.bumps.manager],
    ];
    let signer = &[&manager_seeds[..]];

    let cpi_initialize_poll_accounts = CpiInitializePoll {
        authority: manager.to_account_info(),
        poll: poll.to_account_info(),
        system_program: system_program.to_account_info(),
    };

    let cpi_initialize_poll_ctx = CpiContext::new_with_signer(
        voting_progeam.to_account_info(),
        cpi_initialize_poll_accounts,
        signer,
    );

    initialize_poll(
        cpi_initialize_poll_ctx,
        poll_id,
        start_time,
        end_time,
        poll_name,
        poll_description,
    );
    Ok(())
}
