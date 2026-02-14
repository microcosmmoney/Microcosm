// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::DhcError;
use crate::state::MiningConfig;

#[derive(Accounts)]
pub struct UpdateDhcPrice<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MINING_CONFIG_SEED],
        bump = mining_config.bump,
        has_one = authority @ DhcError::Unauthorized,
    )]
    pub mining_config: Account<'info, MiningConfig>,
}

pub fn handler(ctx: Context<UpdateDhcPrice>, new_price: u64) -> Result<()> {
    require!(new_price > 0, DhcError::InvalidPrice);

    let config = &mut ctx.accounts.mining_config;
    let old_price = config.current_price;

    config.current_price = new_price;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    msg!("MCC price updated");
    msg!("  Old price: ${}", old_price as f64 / 1_000_000.0);
    msg!("  New price: ${}", new_price as f64 / 1_000_000.0);
    msg!("  Updated at: {}", clock.unix_timestamp);

    Ok(())
}
