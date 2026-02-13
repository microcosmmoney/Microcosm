use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::DhcError;
use crate::state::MiningConfig;

#[derive(Accounts)]
pub struct InitializeMiningConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + MiningConfig::INIT_SPACE,
        seeds = [MINING_CONFIG_SEED],
        bump,
    )]
    pub mining_config: Account<'info, MiningConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMiningConfig>, initial_price: u64) -> Result<()> {
    require!(initial_price > 0, DhcError::InvalidPrice);

    let config = &mut ctx.accounts.mining_config;
    let clock = Clock::get()?;

    config.authority = ctx.accounts.authority.key();
    config.current_price = initial_price;
    config.current_phase = 0;
    config.current_mining_rate = INITIAL_MINING_RATE;
    config.total_minted = 0;
    config.team_total = 0;
    config.treasury_total = 0;
    config.mining_pool_total = 0;
    config.total_usdc_paid = 0;
    config.total_cycled = 0;
    config.last_update_timestamp = clock.unix_timestamp;
    config.created_at = clock.unix_timestamp;
    config.is_halted = false;
    config.bump = ctx.bumps.mining_config;

    msg!("Mining config initialized successfully");
    msg!("Initial price: ${}", initial_price as f64 / 1_000_000.0);
    msg!("Initial mining rate: {}x", INITIAL_MINING_RATE as f64 / 100.0);
    msg!("Current phase: Phase {}", config.current_phase);

    Ok(())
}
