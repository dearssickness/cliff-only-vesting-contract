use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVesting<'info>{
    #[account(
       seeds = [b"config_vesting", admin.key().as_ref()],
       bump,
    )]
    pub config_vesting: Account<'info, CliffVestingAccount>,

    #[account(
       seeds = [b"vesting_vault", admin.key().as_ref()],
       bump,
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

pub fn handler(
    ctx: Context<InitializeVesting>, 
    decimals: u8, 
    start_time: u64, 
    cliff_duration: u64
) -> Result<()> {
    let config = &mut ctx.accounts.config_vesting;
    config.vesting_vault = ctx.accounts.vesting_vault.key();
    config.admin = ctx.accounts.admin.key();
    config.authority = ctx.accounts.authority.key();
    config.token_mint = ctx.accounts.token_mint.key();
    config.decimals = decimals;
    config.start_time = start_time;
    config.cliff_duration = cliff_duration;
    Ok(())
}