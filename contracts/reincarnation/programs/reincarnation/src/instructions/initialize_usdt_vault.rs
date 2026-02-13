use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::ReincarnationPool;
use crate::error::ReincarnationError;

pub fn handler(ctx: Context<InitializeUsdtVault>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;

    require!(
        pool.usdt_mint == Pubkey::default() || pool.usdt_vault == Pubkey::default(),
        ReincarnationError::AlreadyInitialized
    );

    pool.usdt_mint = ctx.accounts.usdt_mint.key();
    pool.usdt_vault = ctx.accounts.usdt_vault.key();

    msg!("USDT Vault initialized");
    msg!("USDT Mint: {}", pool.usdt_mint);
    msg!("USDT Vault: {}", pool.usdt_vault);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUsdtVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    pub usdt_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = usdt_mint,
        associated_token::authority = reincarnation_pool,
    )]
    pub usdt_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
