use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
       init,
       payer = admin,
       seeds = [b"config_vesting", admin.key().as_ref()],
       bump,
       space = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1,
    )]
    pub config_vesting: Account<'info, CliffVestingAccount>,

    #[account(
       init,
       payer = admin,
       seeds = [b"vesting_vault", admin.key().as_ref()],
       bump,
       token::mint = token_mint,
       token::authority = authority,
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    
    /// CHECK: This account just used to sign transfers from vesting_vault
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

pub fn handler(_ctx: Context<InitializeAccounts>) -> Result<()> {
    Ok(())
}