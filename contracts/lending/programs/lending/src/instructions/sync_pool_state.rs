use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct SyncPoolState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.authority == authority.key() @ LendingError::NotPoolAuthority
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        constraint = lp_mint.key() == lending_pool.lp_mint @ LendingError::InvalidParameter
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        constraint = vault.key() == lending_pool.vault @ LendingError::InvalidParameter
    )]
    pub vault: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<SyncPoolState>, force_reset_borrowed: bool) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    let lp_mint = &ctx.accounts.lp_mint;
    let vault = &ctx.accounts.vault;
    let clock = Clock::get()?;

    msg!("=== Sync Pool State ===");
    msg!("Pool: {}", pool.key());
    msg!("force_reset_borrowed: {}", force_reset_borrowed);

    msg!("Before sync:");
    msg!("  total_lp_supply (pool): {}", pool.total_lp_supply);
    msg!("  lp_mint.supply (actual): {}", lp_mint.supply);
    msg!("  total_deposits (pool): {}", pool.total_deposits);
    msg!("  total_borrowed (pool): {}", pool.total_borrowed);
    msg!("  vault.amount (actual): {}", vault.amount);

    if pool.total_lp_supply != lp_mint.supply {
        msg!("Fixing total_lp_supply: {} -> {}", pool.total_lp_supply, lp_mint.supply);
        pool.total_lp_supply = lp_mint.supply;
    }

    if force_reset_borrowed && pool.total_borrowed > 0 {
        msg!("FORCE resetting total_borrowed: {} -> 0", pool.total_borrowed);
        pool.total_borrowed = 0;
    }

    if pool.total_borrowed == 0 {
        let expected_deposits = vault.amount
            .saturating_sub(pool.accrued_interest)
            .saturating_add(pool.protocol_fees);

        if pool.total_deposits != expected_deposits {
            msg!("Fixing total_deposits: {} -> {}", pool.total_deposits, expected_deposits);
            pool.total_deposits = expected_deposits;
        }
    } else {
        msg!("Active loans exist ({}), skipping total_deposits sync", pool.total_borrowed);
    }

    pool.updated_at = clock.unix_timestamp;

    msg!("After sync:");
    msg!("  total_lp_supply: {}", pool.total_lp_supply);
    msg!("  total_deposits: {}", pool.total_deposits);
    msg!("  total_borrowed: {}", pool.total_borrowed);

    msg!("=== Sync Complete ===");

    Ok(())
}
