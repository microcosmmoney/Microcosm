use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct PausePool<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.authority == authority.key() @ LendingError::NotPoolAuthority
    )]
    pub lending_pool: Account<'info, LendingPool>,
}

pub fn pause_deposits(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    pool.deposits_paused = true;
    pool.updated_at = Clock::get()?.unix_timestamp;
    msg!("Deposits paused for pool: {}", pool.name);
    Ok(())
}

pub fn unpause_deposits(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    pool.deposits_paused = false;
    pool.updated_at = Clock::get()?.unix_timestamp;
    msg!("Deposits resumed for pool: {}", pool.name);
    Ok(())
}

pub fn pause_borrows(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    pool.borrows_paused = true;
    pool.updated_at = Clock::get()?.unix_timestamp;
    msg!("Borrows paused for pool: {}", pool.name);
    Ok(())
}

pub fn unpause_borrows(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    pool.borrows_paused = false;
    pool.updated_at = Clock::get()?.unix_timestamp;
    msg!("Borrows resumed for pool: {}", pool.name);
    Ok(())
}

pub fn activate_pool(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    pool.is_active = true;
    pool.deposits_paused = false;
    pool.borrows_paused = false;
    pool.updated_at = Clock::get()?.unix_timestamp;
    msg!("Pool activated: {}", pool.name);
    Ok(())
}

pub fn deactivate_pool(ctx: Context<PausePool>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;
    pool.is_active = false;
    pool.updated_at = Clock::get()?.unix_timestamp;
    msg!("Pool deactivated: {}", pool.name);
    Ok(())
}
