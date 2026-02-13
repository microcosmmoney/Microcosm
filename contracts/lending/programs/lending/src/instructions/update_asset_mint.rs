use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct UpdateAssetMint<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.authority == authority.key() @ LendingError::NotPoolAuthority
    )]
    pub lending_pool: Account<'info, LendingPool>,

    pub new_asset_mint: Account<'info, Mint>,
}

pub fn handler(ctx: Context<UpdateAssetMint>) -> Result<()> {
    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;

    require!(
        pool.total_deposits == 0,
        LendingError::PoolHasDeposits
    );

    require!(
        pool.total_borrowed == 0,
        LendingError::PoolHasLoans
    );

    let old_mint = pool.asset_mint;
    let new_mint = ctx.accounts.new_asset_mint.key();

    pool.asset_mint = new_mint;
    pool.updated_at = clock.unix_timestamp;

    msg!("Asset mint updated");
    msg!("Old mint: {}", old_mint);
    msg!("New mint: {}", new_mint);

    Ok(())
}
