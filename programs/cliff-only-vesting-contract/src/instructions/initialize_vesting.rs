use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVesting<'info>{
    #[account(
        mut,
        seeds = [b"config_vesting", token_mint.key().as_ref()],
        bump,
    )]
    pub config_vesting: Account<'info, CliffVestingAccount>,

    #[account(
       seeds = [b"vesting_vault", token_mint.key().as_ref()],
       bump,
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    
    /// CHECK: This account just used to sign transfers from vesting_vault
    #[account(
        seeds = [b"authority", token_mint.key().as_ref()],
        bump,
    )]
    pub authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(mut)]
    pub admin_token_account: Account<'info, TokenAccount>,
    
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>

}

pub fn handler(
    ctx: Context<InitializeVesting>, 
    decimals: u8, 
    start_time: i64, 
    cliff_duration: u64,
    revocable: bool
) -> Result<()> {
    let config = &mut ctx.accounts.config_vesting;
    config.vesting_vault = ctx.accounts.vesting_vault.key();
    config.admin = ctx.accounts.admin.key();
    config.authority = ctx.accounts.authority.key();
    config.token_mint = ctx.accounts.token_mint.key();
    config.decimals = decimals;
    config.start_time = start_time;
    config.cliff_duration = cliff_duration;
    config.revocable = revocable;
    
    Ok(())
}