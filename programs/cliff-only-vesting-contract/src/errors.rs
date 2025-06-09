use anchor_lang::prelude::*;

#[error_code]
pub enum VestingErrors{
    #[msg("Claim before cliff time")]
    EarlyClaim
}