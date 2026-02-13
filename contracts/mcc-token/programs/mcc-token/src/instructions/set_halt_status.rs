use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::DhcError;
use crate::state::MiningConfig;

#[derive(Accounts)]
pub struct SetHaltStatus<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MINING_CONFIG_SEED],
        bump = mining_config.bump,
        has_one = authority @ DhcError::Unauthorized,
    )]
    pub mining_config: Account<'info, MiningConfig>,
}

pub fn handler(ctx: Context<SetHaltStatus>, is_halted: bool) -> Result<()> {
    let config = &mut ctx.accounts.mining_config;
    let old_status = config.is_halted;

    if old_status == is_halted {
        msg!("System halt status unchanged, current status: {}", is_halted);
        return Ok(());
    }

    config.is_halted = is_halted;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    if is_halted {
        msg!("System halted");
        msg!("  Reason: Excessive oracle price deviation");
        msg!("  Halt time: {}", clock.unix_timestamp);
        msg!("  All trading and mining operations suspended");
    } else {
        msg!("System resumed");
        msg!("  Resume time: {}", clock.unix_timestamp);
        msg!("  Trading and mining operations restored");
    }

    Ok(())
}
