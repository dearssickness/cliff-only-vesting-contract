#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

pub mod instructions;
pub mod state;
pub mod errors;

use state::*;
use instructions::*;

declare_id!("J3Qo8zJpx3cj6PW8Zru1xTbR2WFPLmy8rJMyoctKNiVf");

#[program]
pub mod cliff_only_vesting_contract {
    use super::*;

    pub fn initialize_accounts(ctx: Context<InitializeAccounts>) -> Result<()> {
        instructions::initialize_accounts::handler(ctx)
    }
    
    pub fn add_beneficiary(ctx: Context<AddBeneficiary>, total_tokens: u64) -> Result<()>{
        instructions::add_beneficiary::handler(ctx, total_tokens)
    }
    
    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>,
        decimals: u8,
        start_time: i64,
        cliff_duration: u64,
        revocable: bool
    ) -> Result<()> {
        instructions::initialize_vesting::handler(
            ctx, 
            decimals, 
            start_time, 
            cliff_duration, 
            revocable
        )
    }
    
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim::handler(ctx)
    }
    
    pub fn revoke(ctx: Context<Revoke>) -> Result<()> {
        instructions::revoke::handler(ctx)
    }

    pub fn initialize_beneficiary_account(
        ctx: Context<InitializeBeneficiaryAccount>
    ) -> Result<()> {
        instructions::initialize_beneficiary_account::handler(ctx)
    }
}