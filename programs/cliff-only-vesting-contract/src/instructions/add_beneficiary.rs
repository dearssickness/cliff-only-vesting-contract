use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
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

    #[account(mut)]
    pub beneficiary_wallet: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<AddBeneficiary>, total_tokens: u64) -> Result<()> {
    let beneficiary_data = &mut ctx.accounts.beneficiary_data;
    beneficiary_data.beneficiary_wallet = ctx.accounts.beneficiary_wallet.key();
    beneficiary_data.total_tokens = total_tokens;
    
    Ok(()) 
}