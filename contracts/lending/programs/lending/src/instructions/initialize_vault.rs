// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::LendingPool;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    pub asset_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = asset_mint,
        associated_token::authority = lending_pool
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;

    pool.vault = ctx.accounts.vault.key();

    msg!("Vault initialized for lending pool");
    msg!("Vault address: {}", pool.vault);

    Ok(())
}
