use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::constants::*;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct DebugPool<'info> {
    #[account(
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        constraint = lp_mint.key() == lending_pool.lp_mint
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        constraint = vault.key() == lending_pool.vault
    )]
    pub vault: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<DebugPool>) -> Result<()> {
    let pool = &ctx.accounts.lending_pool;
    let lp_mint = &ctx.accounts.lp_mint;
    let vault = &ctx.accounts.vault;

    msg!("=== DEBUG POOL STATE ===");
    msg!("Pool key: {}", pool.key());
    msg!("Pool name: {}", pool.name);
    msg!("Authority: {}", pool.authority);
    msg!("Asset mint: {}", pool.asset_mint);
    msg!("LP mint: {}", pool.lp_mint);
    msg!("Vault: {}", pool.vault);
    msg!("");
    msg!("=== POOL VALUES ===");
    msg!("total_deposits: {}", pool.total_deposits);
    msg!("total_borrowed: {}", pool.total_borrowed);
    msg!("total_lp_supply: {}", pool.total_lp_supply);
    msg!("accrued_interest: {}", pool.accrued_interest);
    msg!("protocol_fees: {}", pool.protocol_fees);
    msg!("");
    msg!("=== ACTUAL VALUES ===");
    msg!("lp_mint.supply: {}", lp_mint.supply);
    msg!("vault.amount: {}", vault.amount);
    msg!("");
    msg!("=== COMPUTED ===");
    msg!("available_liquidity: {}", pool.available_liquidity());
    msg!("lp_exchange_rate: {}", pool.lp_exchange_rate());

    let lp_500 = 500_000_000_000u64;
    let asset_for_500 = pool.lp_to_asset(lp_500);
    msg!("lp_to_asset(500 LP): {}", asset_for_500);

    msg!("");
    msg!("=== COMPARISON ===");
    msg!("pool.total_lp_supply == lp_mint.supply: {}", pool.total_lp_supply == lp_mint.supply);
    msg!("asset_for_500 <= available_liquidity: {}", asset_for_500 <= pool.available_liquidity());

    Ok(())
}
