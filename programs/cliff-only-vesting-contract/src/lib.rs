#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("J3Qo8zJpx3cj6PW8Zru1xTbR2WFPLmy8rJMyoctKNiVf");

#[program]
pub mod cliff_only_vesting_contract {
    use super::*;

    pub fn initialize_accounts(_ctx: Context<InitializeAccounts>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
       init,
       payer = admin,
       seeds = [b"vesting_vault", admin.key().as_ref()],
       bump,
       token::mint = token_mint,
       token::authority = authority,
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    
    /// CHECK: this account just used to sign transfers from vesting_vault
    #[account(
        seeds = [b"authority", admin.key().as_ref()],
        bump,
    )]
    pub authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
    
}
