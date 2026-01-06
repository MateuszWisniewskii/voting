use anchor_lang::prelude::*;
use crate::{Poll, Candidate};

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct ResolvePoll<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // To będzie nasz Manager PDA

    #[account(
        mut,
        seeds = [b"poll_seed".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
        close = receiver, // Zamknięcie konta Poll i zwrot SOL
        constraint = poll.authority == authority.key(),
        constraint = Clock::get()?.unix_timestamp > poll.end_time as i64
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>, // Tu trafią wszystkie odzyskane SOL
}

pub fn handler(ctx: Context<ResolvePoll>, _poll_id: u64) -> Result<()> {
    let poll = &ctx.accounts.poll;
    
    msg!("--- WYNIKI GŁOSOWANIA: {} ---", poll.poll_name);

    let mut winner_name = String::from("Brak głosów");
    let mut max_votes = 0;
    let mut is_tie = false;

for candidate_info in ctx.remaining_accounts.iter() {
        let mut data: &[u8] = &candidate_info.try_borrow_data()?;
        let candidate = Candidate::try_deserialize(&mut data)?;

        msg!("Kandydat: {} | Głosy: {}", candidate.candidate_name, candidate.candidate_votes);

        // Wyłanianie zwycięzcy
        if candidate.candidate_votes > max_votes {
            max_votes = candidate.candidate_votes;
            winner_name = candidate.candidate_name.clone();
            is_tie = false;
        } else if candidate.candidate_votes == max_votes && max_votes > 0 {
            is_tie = true;
        }

        // ZAMKNIĘCIE KONTA I ZWROT RENTU
        let candidate_lamports = candidate_info.lamports();
        let receiver_lamports = ctx.accounts.receiver.lamports();

        // Ręczne przeniesienie środków (standard w Solana przy zamykaniu kont w pętli)
        **ctx.accounts.receiver.lamports.borrow_mut() = receiver_lamports
            .checked_add(candidate_lamports)
            .expect("Błąd przelewu lamportów"); // To się nie wydarzy przy SOL
            
        **candidate_info.lamports.borrow_mut() = 0;
    }

    if is_tie {
        msg!("WYNIK: Remis! Najwyższa liczba głosów to: {}", max_votes);
    } else if max_votes == 0 {
        msg!("WYNIK: Nikt nie oddał głosów.");
    } else {
        msg!("ZWYCIĘZCA: {} z liczbą głosów: {}", winner_name, max_votes);
    }

    msg!("Wszystkie konta zostały zamknięte, Rent zwrócony.");
    Ok(())
}