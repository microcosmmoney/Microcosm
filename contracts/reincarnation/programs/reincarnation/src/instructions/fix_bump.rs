// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::REINCARNATION_POOL_SEED;

pub fn handler(ctx: Context<FixBump>) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;
    let bump = ctx.bumps.reincarnation_pool;

    msg!("Fixing bump value");
    msg!("Correct bump: {}", bump);

    let mut data = pool.try_borrow_mut_data()?;

    let bump_offset = 467;
    msg!("Bump offset (fixed): {}", bump_offset);
    msg!("Current bump value: {}", data[bump_offset]);

    data[bump_offset] = bump;

    msg!("Bump fixed to: {}", bump);

    Ok(())
}

#[derive(Accounts)]
pub struct FixBump<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,
}
