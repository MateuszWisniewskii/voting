use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("Nie można dodawać kandydatów po rozpoczęciu głosowania")]
    AddingCandidateAfterVotingStart,
}
