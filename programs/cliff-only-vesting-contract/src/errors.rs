use anchor_lang::prelude::*;

#[error_code]
pub enum VestingErrors{
    #[msg("Claim before cliff time")]
    EarlyClaim,
    #[msg("Vesting is not revocable")]
    NotRevocable,
    #[msg("This account already claimed tokens")]
    AlreadyClaimed,
    #[msg("Tokens don't match")]
    InvalidMint,
    #[msg("Beneficiary owner and beneficiary key that passed in, don't match")]
    InvalidBeneficiary
}