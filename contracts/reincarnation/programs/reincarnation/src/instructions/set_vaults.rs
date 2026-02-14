// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(ctx: Context<SetVaults>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;

    pool.usdc_vault = ctx.accounts.usdc_vault.key();
    pool.mcc_vault = ctx.accounts.mcc_vault.key();
    pool.mcd_vault = ctx.accounts.mcd_vault.key();

    msg!("Vaults set successfully");
    msg!("USDC Vault: {}", pool.usdc_vault);
    msg!("MCC Vault: {}", pool.mcc_vault);
    msg!("MCD Vault: {}", pool.mcd_vault);

    Ok(())
}

#[derive(Accounts)]
pub struct SetVaults<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = usdc_vault.owner == reincarnation_pool.key(),
        constraint = usdc_vault.mint == reincarnation_pool.usdc_mint @ ReincarnationError::InvalidUsdcMint,
    )]
    pub usdc_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = mcc_vault.owner == reincarnation_pool.key(),
        constraint = mcc_vault.mint == reincarnation_pool.mcc_mint @ ReincarnationError::InvalidMccMint,
    )]
    pub mcc_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = mcd_vault.owner == reincarnation_pool.key(),
        constraint = mcd_vault.mint == reincarnation_pool.mcd_mint @ ReincarnationError::InvalidMccMint,
    )]
    pub mcd_vault: Account<'info, TokenAccount>,
}
