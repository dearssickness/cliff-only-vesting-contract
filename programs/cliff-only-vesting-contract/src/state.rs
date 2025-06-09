use anchor_lang::prelude::*;

#[account]
pub struct BeneficiaryData {
    pub beneficiary_wallet: Pubkey,
    pub total_tokens: u64,
}

#[account]
pub struct CliffVestingAccount {
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub vesting_vault: Pubkey,
    pub cliff_duration: u64,
    pub start_time: i64,
    pub decimals: u8,
    pub revocable: bool,
}