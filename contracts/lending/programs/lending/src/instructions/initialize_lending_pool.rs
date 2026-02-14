// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
#[instruction(pool_name: String)]
pub struct InitializeLendingPool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = LendingPool::LEN,
        seeds = [LENDING_POOL_SEED, pool_name.as_bytes()],
        bump
    )]
    pub lending_pool: Account<'info, LendingPool>,

    pub asset_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeLendingPool>,
    pool_name: String,
    base_rate: u64,
    optimal_utilization: u64,
    slope1: u64,
    slope2: u64,
) -> Result<()> {
    require!(
        pool_name.len() <= MAX_POOL_NAME_LEN,
        LendingError::PoolNameTooLong
    );

    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;

    pool.name = pool_name;
    pool.authority = ctx.accounts.authority.key();
    pool.asset_mint = ctx.accounts.asset_mint.key();
    pool.lp_mint = Pubkey::default();
    pool.vault = Pubkey::default();

    pool.total_deposits = 0;
    pool.total_borrowed = 0;
    pool.total_lp_supply = 0;
    pool.accrued_interest = 0;
    pool.protocol_fees = 0;

    pool.base_rate = if base_rate > 0 { base_rate } else { DEFAULT_BASE_RATE };
    pool.optimal_utilization = if optimal_utilization > 0 { optimal_utilization } else { DEFAULT_OPTIMAL_UTILIZATION };
    pool.slope1 = if slope1 > 0 { slope1 } else { DEFAULT_SLOPE1 };
    pool.slope2 = if slope2 > 0 { slope2 } else { DEFAULT_SLOPE2 };

    pool.is_active = true;
    pool.deposits_paused = false;
    pool.borrows_paused = false;

    pool.last_update_timestamp = clock.unix_timestamp;
    pool.created_at = clock.unix_timestamp;
    pool.updated_at = clock.unix_timestamp;
    pool.bump = ctx.bumps.lending_pool;

    msg!("Lending pool initialized: {}", pool.name);
    msg!("Base rate: {} BPS", pool.base_rate);
    msg!("Optimal utilization: {} BPS", pool.optimal_utilization);
    msg!("Note: Run initialize_lp_mint, initialize_vault, and initialize_nft_oracle next");

    Ok(())
}
