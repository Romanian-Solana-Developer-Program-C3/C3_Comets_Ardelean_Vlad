use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Maximum stake reached")]
    MaxStakeReached,
}
