use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, McdWhitelist};

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct UpdateWhitelistStatus<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MCD_CONFIG_SEED],
        bump = mcd_config.bump,
        constraint = mcd_config.authority == authority.key() @ McdError::Unauthorized,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    #[account(
        mut,
        seeds = [MCD_WHITELIST_SEED, &project_id.to_le_bytes()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, McdWhitelist>,
}

pub fn handler(
    ctx: Context<UpdateWhitelistStatus>,
    project_id: u64,
    new_status: u8,
) -> Result<()> {
    require!(
        new_status == McdWhitelist::STATUS_ACTIVE || new_status == McdWhitelist::STATUS_SUSPENDED,
        McdError::InvalidStatus
    );

    let whitelist = &mut ctx.accounts.whitelist;
    let clock = Clock::get()?;

    whitelist.status = new_status;
    whitelist.updated_at = clock.unix_timestamp;

    let status_str = if new_status == McdWhitelist::STATUS_ACTIVE {
        "active"
    } else {
        "suspended"
    };

    msg!("Updated whitelist status");
    msg!("Project ID: {}", project_id);
    msg!("New Status: {}", status_str);

    Ok(())
}
