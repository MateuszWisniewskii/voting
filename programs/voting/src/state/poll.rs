use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub authority: Pubkey,
    #[max_len(40)]
    pub poll_name: String, // Krótka nazwa
    #[max_len(200)]
    pub poll_description: String, // Opis czego dotyczy głosowanie
    pub start_time: u64, // UnixTimeStamp -> moment rozpoczęcia głosowania
    pub end_time: u64, // UnixTimeStamp -> koniec głosowania
    pub poll_option_index: u64, // Index trzymający ilość kandydatów
    #[max_len(20)]
    pub winning_candidate: String, 
}