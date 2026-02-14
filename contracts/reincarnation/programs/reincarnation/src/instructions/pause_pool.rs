// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler_pause(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;
    pool.paused = true;

    let clock = Clock::get()?;
    msg!("Pool paused by {}", ctx.accounts.authority.key());

    emit!(PoolPaused {
        paused_by: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

pub fn handler_unpause(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;
    pool.paused = false;

    let clock = Clock::get()?;
    msg!("Pool unpaused by {}", ctx.accounts.authority.key());

    emit!(PoolUnpaused {
        unpaused_by: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct PausePool<'info> {
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
pub struct PoolPaused {
    pub paused_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct PoolUnpaused {
    pub unpaused_by: Pubkey,
    pub timestamp: i64,
}
