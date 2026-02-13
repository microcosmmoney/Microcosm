use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(
    ctx: Context<InitializePool>,
    pool_name: String,
    base_price: u64,
    premium_bps: u64,
    daily_limit: u64,
) -> Result<()> {
    require!(
        pool_name.len() <= MAX_POOL_NAME_LEN,
        ReincarnationError::PoolNameTooLong
    );
    require!(
        !pool_name.is_empty(),
        ReincarnationError::InvalidPoolName
    );

    require!(
        base_price >= MIN_BASE_PRICE && base_price <= MAX_BASE_PRICE,
        ReincarnationError::InvalidPrice
    );

    let pool = &mut ctx.accounts.reincarnation_pool;
    let clock = Clock::get()?;

    pool.name = pool_name;
    pool.authority = ctx.accounts.authority.key();
    pool.mcc_mint = ctx.accounts.mcc_mint.key();
    pool.usdc_mint = ctx.accounts.usdc_mint.key();
    pool.mcd_mint = ctx.accounts.mcd_mint.key();

    pool.usdc_vault = Pubkey::default();
    pool.mcc_vault = Pubkey::default();
    pool.mcd_vault = Pubkey::default();

    pool.usdt_mint = Pubkey::default();
    pool.usdt_vault = Pubkey::default();

    pool.base_price = base_price;
    pool.premium_bps = if premium_bps == 0 { BUYBACK_PREMIUM_BPS } else { premium_bps };
    pool.price_updated_at = clock.unix_timestamp;

    pool.paused = false;
    pool.daily_limit = if daily_limit == 0 { DAILY_LIMIT_USDC } else { daily_limit };
    pool.daily_used = 0;
    pool.last_reset_day = clock.unix_timestamp / 86400;

    pool.total_mcc_bought = 0;
    pool.total_usd_paid = 0;
    pool.total_buyback_count = 0;

    pool.total_mining_usd_received = 0;
    pool.total_mining_mcc_minted = 0;
    pool.total_mining_mcd_minted = 0;
    pool.total_mining_count = 0;

    pool.last_cycle_timestamp = 0;
    pool.total_mcc_cycled = 0;
    pool.total_mcd_cycled = 0;
    pool.total_cycle_count = 0;

    pool.created_at = clock.unix_timestamp;
    pool.bump = ctx.bumps.reincarnation_pool;

    msg!("Reincarnation pool initialized: {}", pool.name);
    msg!("Base price: {} USDC (6 decimals)", pool.base_price);
    msg!("Premium: {} bps", pool.premium_bps);
    msg!("Daily limit: {} USDC", pool.daily_limit);

    Ok(())
}

#[derive(Accounts)]
#[instruction(pool_name: String)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ReincarnationPool::LEN,
        seeds = [REINCARNATION_POOL_SEED],
        bump
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    pub mcc_mint: UncheckedAccount<'info>,

    pub usdc_mint: UncheckedAccount<'info>,

    pub mcd_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
