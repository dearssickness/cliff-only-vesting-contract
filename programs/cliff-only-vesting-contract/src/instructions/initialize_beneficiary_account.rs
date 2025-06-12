use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeBeneficiaryAccount<'info>{
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

    pub system_program: Program<'info, System>
}

pub fn handler(
    _ctx: Context<InitializeBeneficiaryAccount>
) -> Result<()>{
    Ok(())
}