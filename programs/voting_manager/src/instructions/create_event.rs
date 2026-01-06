use anchor_lang::prelude::*;

use voting::{
    cpi::{
        accounts::{
            InitializeCandidate as CpiInitializeCandidate, 
            InitializePoll as CpiInitializePoll,
        },
        initialize_candidate, // <-- POPRAWIONO (było cadidate)
        initialize_poll,
    },
    program::Voting,
};

use crate::{error::VotingManagerError, Manager};

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
    /// Klient JS musi wyliczyć poprawny adres PDA
    #[account(mut)]
    pub poll: UncheckedAccount<'info>,

    #[account()]
    pub voting_program: Program<'info, Voting>,

    #[account()]
    pub system_program: Program<'info, System>,
}

//ctx: Context<CreateEvent>,
pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateEvent<'info>>,
    start_time: u64,
    end_time: u64,
    poll_name: String,
    poll_description: String,
    candidate_names: Vec<String>,
) -> Result<()> {
    let manager = &mut ctx.accounts.manager;
    let poll_id = manager.poll_count;
    let poll_info = ctx.accounts.poll.to_account_info();
    let authority_info = ctx.accounts.authority.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();
    let voting_program = ctx.accounts.voting_program.to_account_info();

    // 1. Przygotowanie Signera (Manager PDA musi podpisać CPI)
    let authority_key = ctx.accounts.authority.key();
    let manager_seeds = &[
        b"manager_seed".as_ref(),
        authority_key.as_ref(),
        &[ctx.bumps.manager],
    ];
    let signer = &[&manager_seeds[..]];

    // 2. CPI: Inicjalizacja Poll
    // Tutaj przekazujemy 'payer' jako authority (użytkownik), a 'authority' jako manager (PDA)
    let cpi_accounts_poll = CpiInitializePoll {
        payer: authority_info.clone(),           // Użytkownik płaci (Signer)
        authority: manager.to_account_info(),    // Manager autoryzuje (PDA)
        poll: poll_info.clone(),
        system_program: system_program.clone(),
    };

    let cpi_ctx_poll = CpiContext::new_with_signer(
        voting_program.clone(),
        cpi_accounts_poll,
        signer,
    );

    initialize_poll(
        cpi_ctx_poll,
        poll_id,
        start_time,
        end_time,
        poll_name,
        poll_description,
    )?;

    // 3. CPI: Inicjalizacja Kandydatów
    let candidates_accounts = ctx.remaining_accounts;

    require!(
        candidate_names.len() == candidates_accounts.len(),
        VotingManagerError::CandidateCountMismatch
    );

    for (i, candidate_name) in candidate_names.iter().enumerate() {
        let candidate_account_info = &candidates_accounts[i];

        let cpi_accounts_candidate = CpiInitializeCandidate {
            payer: authority_info.clone(),        // Użytkownik płaci
            authority: manager.to_account_info(), // Manager autoryzuje
            candidate: candidate_account_info.to_account_info(),
            poll: poll_info.clone(),
            system_program: system_program.clone(),
        };

        let cpi_ctx_candidate = CpiContext::new_with_signer(
            voting_program.clone(),
            cpi_accounts_candidate,
            signer,
        );

        initialize_candidate(
            cpi_ctx_candidate, 
    poll_id, 
    candidate_name.clone()
        )?;
    }
    
    // 4. Inkrementacja licznika
    manager.poll_count = manager.poll_count
        .checked_add(1)
        .ok_or(VotingManagerError::Overflow)?;

    Ok(())
}
