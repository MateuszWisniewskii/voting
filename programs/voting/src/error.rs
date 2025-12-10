use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("Nie można dodawać kandydatów po rozpoczęciu głosowania")]
    AddingCandidateAfterVotingStart,
    #[msg("Nie można oddać głosu więcej niż raz")]
    VoteHaveBeenPlaced,
    #[msg("Głosowani jeszcze się nie zaczęło")]
    VotingNotStarted,
    #[msg("Głosowanie się zakończyło")]
    VotingEnded,
    #[msg("zmienna przepełniła się. Overflow occured")]
    Overflow,
}
