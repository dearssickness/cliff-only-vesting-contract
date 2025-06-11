use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{errors::*, state::*};

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(
       seeds = [b"config_vesting", token_mint.key().as_ref()],
       bump,
    )]
    pub config_vesting: Account<'info, CliffVestingAccount>,

    #[account(
        mut,
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

    #[account(
        seeds = [b"beneficiary_data", beneficiary_wallet.key().as_ref()],
        bump,
    )]
    pub beneficiary_data: Account<'info, BeneficiaryData>,

    #[account(mut)]
    pub beneficiary_wallet: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>

}

pub fn handler(ctx: Context<Claim>) -> Result<()>{
    
    let config = &ctx.accounts.config_vesting;
    let start_time = config.start_time;
    let cliff_duration = config.cliff_duration;
    let cliff_time = start_time + cliff_duration as i64;
    let clock = Clock::get()?;
    
    require!(clock.unix_timestamp > cliff_time, VestingErrors::EarlyClaim);
 
    
    let token_mint_key = &ctx.accounts.token_mint.key();
    let token_program = ctx.accounts.token_program.to_account_info();

    let beneficiary_data = &ctx.accounts.beneficiary_data;
    let total_beneficiary_tokens = beneficiary_data.total_tokens;

    
    let authority_seeds = &[
        b"authority", 
        token_mint_key.as_ref(),
        &[ctx.bumps.authority]];
    let signer = &[&authority_seeds[..]];
    
    let transfer = token::Transfer{
       from: ctx.accounts.vesting_vault.to_account_info(),
       to: ctx.accounts.beneficiary_wallet.to_account_info(),
       authority: ctx.accounts.authority.to_account_info()
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        token_program, 
        transfer, 
        signer);
    token::transfer(cpi_ctx, total_beneficiary_tokens)?;

    Ok(())
}