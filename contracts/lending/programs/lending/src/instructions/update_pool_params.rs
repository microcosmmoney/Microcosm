use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct UpdatePoolParams<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.authority == authority.key() @ LendingError::NotPoolAuthority
    )]
    pub lending_pool: Account<'info, LendingPool>,
}

pub fn handler(
    ctx: Context<UpdatePoolParams>,
    new_base_rate: Option<u64>,
    new_optimal_utilization: Option<u64>,
    new_slope1: Option<u64>,
    new_slope2: Option<u64>,
) -> Result<()> {
    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;

    pool.accrue_interest(clock.unix_timestamp)?;

    if let Some(base_rate) = new_base_rate {
        require!(base_rate <= 5000, LendingError::InvalidParameter);
        pool.base_rate = base_rate;
        msg!("Base rate updated to {} BPS", base_rate);
    }

    if let Some(optimal_utilization) = new_optimal_utilization {
        require!(optimal_utilization <= 9500, LendingError::InvalidParameter);
        require!(optimal_utilization >= 5000, LendingError::InvalidParameter);
        pool.optimal_utilization = optimal_utilization;
        msg!("Optimal utilization updated to {} BPS", optimal_utilization);
    }

    if let Some(slope1) = new_slope1 {
        require!(slope1 <= 2000, LendingError::InvalidParameter);
        pool.slope1 = slope1;
        msg!("Slope1 updated to {} BPS", slope1);
    }

    if let Some(slope2) = new_slope2 {
        require!(slope2 <= 30000, LendingError::InvalidParameter);
        pool.slope2 = slope2;
        msg!("Slope2 updated to {} BPS", slope2);
    }

    pool.updated_at = clock.unix_timestamp;

    msg!("Pool parameters updated");
    msg!("New borrow rate: {} BPS", pool.borrow_rate());
    msg!("New supply rate: {} BPS", pool.supply_rate());

    Ok(())
}
