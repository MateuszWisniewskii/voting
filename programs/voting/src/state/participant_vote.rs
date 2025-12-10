use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ParticipantVote {
    pub participant: Pubkey,
    pub poll_id: u64,
    #[max_len(20)]
    pub candidate_name: String, // Nazwa kandydata na, którego został oddany głos
    pub vote_placed: bool,
}