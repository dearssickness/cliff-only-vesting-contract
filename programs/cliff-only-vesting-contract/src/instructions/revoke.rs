use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{state::*, errors::*};

#[derive(Accounts)]
pub struct Revoke<'info>{
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

    #[account(mut)]
    pub admin_token_account: Account<'info, TokenAccount>,
 
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
    
}

pub fn handler(ctx: Context<Revoke>) -> Result<()> {
    let config = &ctx.accounts.config_vesting;
    require!(config.revocable == true, VestingErrors::NotRevocable);
    
    let amount = ctx.accounts.vesting_vault.amount;
    
    let token_program = ctx.accounts.token_program.to_account_info();
    
    let token_mint_key = ctx.accounts.token_mint.key();
    let authority_seeds = [
        b"authority", 
        token_mint_key.as_ref(), 
        &[ctx.bumps.authority]
        ];
    
    let signer = &[&authority_seeds[..]];
    
    let transfer = token::Transfer{
        from: ctx.accounts.vesting_vault.to_account_info(),
        to: ctx.accounts.admin_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info()
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        token_program, 
        transfer, 
        signer
    );
    
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}
    