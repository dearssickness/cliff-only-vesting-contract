use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::*;

#[derive(Accounts)]
pub struct AddBeneficiary <'info> {
    #[account(
       init,
       payer = admin,
       seeds = [b"beneficiary_data", beneficiary_wallet.key().as_ref()],
       bump,
       space = 8 + 32 + 8 + 8,
    )]
    pub beneficiary_data: Account<'info, BeneficiaryData>,

    #[account(
       seeds = [b"vesting_vault", token_mint.key().as_ref()],
       bump,
    )]
    pub vesting_vault: Account<'info, TokenAccount>,

    #[account(
       seeds = [b"config_vesting", token_mint.key().as_ref()],
       bump,
    )]
    pub config_vesting: Account<'info, CliffVestingAccount>,

    #[account(mut)]
    pub beneficiary_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<AddBeneficiary>, 
    total_tokens: u64,
) -> Result<()> {
    let beneficiary_data = &mut ctx.accounts.beneficiary_data;
    beneficiary_data.beneficiary_wallet = ctx.accounts.beneficiary_wallet.key();
    beneficiary_data.total_tokens = total_tokens;
    
    let decimals = ctx.accounts.config_vesting.decimals;

    let token_program = ctx.accounts.token_program.to_account_info();
    
    let transfer = token::Transfer {
        from: ctx.accounts.beneficiary_wallet.to_account_info(),
        to: ctx.accounts.vesting_vault.to_account_info(),
        authority: ctx.accounts.admin.to_account_info()
    };
    
    let cpi_ctx = CpiContext::new(token_program, transfer);
    
    token::transfer(cpi_ctx, total_tokens * u64::pow(10, decimals as u32))?;
   
    Ok(()) 
}