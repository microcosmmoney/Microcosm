// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::FragmentConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = FragmentConfig::LEN,
        seeds = [FRAGMENT_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, FragmentConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeConfig>) -> Result<()> {
    let clock = Clock::get()?;
    let config = &mut ctx.accounts.config;

    config.authority = ctx.accounts.authority.key();
    config.initialize_defaults();
    config.created_at = clock.unix_timestamp;
    config.updated_at = clock.unix_timestamp;
    config.bump = ctx.bumps.config;

    msg!("Fragment protocol initialized!");
    msg!("Authority: {}", config.authority);
    msg!("Min fragments: {}", config.min_fragments);
    msg!("Max fragments: {}", config.max_fragments);
    msg!("Buyout duration: {} seconds", config.buyout_duration);

    Ok(())
}
