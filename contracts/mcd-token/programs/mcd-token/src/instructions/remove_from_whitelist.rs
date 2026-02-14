// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, McdWhitelist};

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct RemoveFromWhitelist<'info> {
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
        close = authority,
        seeds = [MCD_WHITELIST_SEED, &project_id.to_le_bytes()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, McdWhitelist>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RemoveFromWhitelist>,
    project_id: u64,
) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;

    msg!("Removed from MCD whitelist");
    msg!("Project ID: {}", project_id);
    msg!("Wallet: {}", whitelist.wallet_address);

    Ok(())
}
