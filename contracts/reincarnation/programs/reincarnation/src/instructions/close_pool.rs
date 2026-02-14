// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(ctx: Context<ClosePool>) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;

    require!(pool.paused, ReincarnationError::PoolNotPaused);

    msg!("Closing reincarnation pool: {}", pool.name);
    msg!("Total MCC bought: {}", pool.total_mcc_bought);
    msg!("Total USD paid: {}", pool.total_usd_paid);
    msg!("Total buyback count: {}", pool.total_buyback_count);

    Ok(())
}

#[derive(Accounts)]
pub struct ClosePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,
}
