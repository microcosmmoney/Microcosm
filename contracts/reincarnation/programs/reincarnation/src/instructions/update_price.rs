// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(
    ctx: Context<UpdatePrice>,
    new_base_price: u64,
) -> Result<()> {
    require!(
        new_base_price >= MIN_BASE_PRICE,
        ReincarnationError::PriceBelowMinimum
    );
    require!(
        new_base_price <= MAX_BASE_PRICE,
        ReincarnationError::PriceExceedsMaximum
    );

    let pool = &mut ctx.accounts.reincarnation_pool;
    let old_price = pool.base_price;
    let clock = Clock::get()?;

    pool.base_price = new_base_price;
    pool.price_updated_at = clock.unix_timestamp;

    msg!("Price updated:");
    msg!("  Old price: {} USDC (6 decimals)", old_price);
    msg!("  New price: {} USDC (6 decimals)", new_base_price);
    msg!("  Buyback price: {} USDC", pool.calculate_buyback_price());

    emit!(PriceUpdated {
        old_price,
        new_price: new_base_price,
        updated_by: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,
}

#[event]
pub struct PriceUpdated {
    pub old_price: u64,
    pub new_price: u64,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}
