// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::constants::*;
use crate::state::McdConfig;

#[derive(Accounts)]
pub struct InitializeMcdConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = McdConfig::LEN,
        seeds = [MCD_CONFIG_SEED],
        bump,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    pub mcd_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [MCD_GENESIS_POOL_SEED],
        bump,
    )]
    pub genesis_pool: UncheckedAccount<'info>,

    #[account(
        seeds = [MCD_RECYCLE_POOL_SEED],
        bump,
    )]
    pub recycle_pool: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeMcdConfig>) -> Result<()> {
    let config = &mut ctx.accounts.mcd_config;
    let clock = Clock::get()?;

    config.authority = ctx.accounts.authority.key();
    config.mcd_mint = ctx.accounts.mcd_mint.key();
    config.genesis_pool = ctx.accounts.genesis_pool.key();
    config.recycle_pool = ctx.accounts.recycle_pool.key();
    config.total_minted = 0;
    config.total_recycled = 0;
    config.total_vault_minted = 0;
    config.total_consumed = 0;
    config.last_update_timestamp = clock.unix_timestamp;
    config.bump = ctx.bumps.mcd_config;

    msg!("Initialize MCD Config");
    msg!("Authority: {}", config.authority);
    msg!("MCD Mint: {}", config.mcd_mint);
    msg!("Genesis Pool: {}", config.genesis_pool);
    msg!("Recycle Pool: {}", config.recycle_pool);

    Ok(())
}
