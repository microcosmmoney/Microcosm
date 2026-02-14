// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::MiningConfig;

#[derive(Accounts)]
pub struct GetMiningStats<'info> {
    #[account(
        seeds = [MINING_CONFIG_SEED],
        bump = mining_config.bump,
    )]
    pub mining_config: Account<'info, MiningConfig>,
}

pub fn handler(ctx: Context<GetMiningStats>) -> Result<()> {
    let config = &ctx.accounts.mining_config;

    msg!("MCC Mining Statistics");
    msg!("==========================================");
    msg!("Current price: ${}", config.current_price as f64 / 1_000_000.0);
    msg!("Mining phase: Phase {}/{}", config.current_phase, TOTAL_PHASES - 1);
    msg!("Mining rate: {}x", config.current_mining_rate as f64 / 100.0);
    msg!("==========================================");
    msg!("Total minted: {} MCC", config.total_minted as f64 / 1_000_000_000.0);
    msg!("Distribution stats:");
    msg!("  - Team total (20%): {} MCC", config.team_total as f64 / 1_000_000_000.0);
    msg!("  - Treasury total (20%): {} MCC", config.treasury_total as f64 / 1_000_000_000.0);
    msg!("  - Mining pool total (60%): {} MCC", config.mining_pool_total as f64 / 1_000_000_000.0);
    msg!("==========================================");
    msg!("Cumulative USDC payments: ${}", config.total_usdc_paid as f64 / 1_000_000.0);
    msg!("Last updated: {}", config.last_update_timestamp);
    msg!("Created at: {}", config.created_at);
    msg!("==========================================");

    let progress_pct = (config.total_minted as f64 / TOTAL_SUPPLY as f64) * 100.0;
    msg!("Mining progress: {:.2}%", progress_pct);

    let phase_minted = config.total_minted % PHASE_THRESHOLD;
    let phase_progress = (phase_minted as f64 / PHASE_THRESHOLD as f64) * 100.0;
    msg!("Current phase progress: {:.2}%", phase_progress);

    Ok(())
}
