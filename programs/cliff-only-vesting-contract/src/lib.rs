#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

pub mod instructions;
pub mod state;

use state::*;
use instructions::*;

declare_id!("J3Qo8zJpx3cj6PW8Zru1xTbR2WFPLmy8rJMyoctKNiVf");

#[program]
pub mod cliff_only_vesting_contract {
    use super::*;

    pub fn initialize_accounts(ctx: Context<InitializeAccounts>) -> Result<()> {
        instructions::initialize_accounts::handler(ctx)
    }
}