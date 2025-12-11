use anchor_lang::prelude::*;

#[error_code]
pub enum VotingManagerError {
    #[msg("Liczba nazw kandydatów, nie zgadza się z liczbą kont")]
    CandidateCountMismatch,
    #[msg("zmienna przepełniła się. Overflow occured")]
    Overflow,
}
