// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::VerifierConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VerifierConfig::SPACE,
        seeds = [VERIFIER_CONFIG_SEED],
        bump
    )]
    pub verifier_config: Account<'info, VerifierConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeConfig>,
    territory_nft_program: Pubkey,
    mcc_token_program: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.verifier_config;
    let clock = Clock::get()?;

    config.authority = ctx.accounts.authority.key();
    config.territory_nft_program = territory_nft_program;
    config.mcc_token_program = mcc_token_program;
    config.total_users = 0;
    config.level_1_count = 0;
    config.level_2_count = 0;
    config.level_3_count = 0;
    config.level_4_count = 0;
    config.level_5_count = 0;
    config.level_6_count = 0;
    config.level_7_count = 0;
    config.is_paused = false;
    config.is_initialized = true;
    config.created_at = clock.unix_timestamp;
    config.updated_at = clock.unix_timestamp;
    config._reserved = [0u8; 64];

    msg!("VerifierConfig initialized");
    msg!("Authority: {}", config.authority);
    msg!("Territory NFT Program: {}", config.territory_nft_program);
    msg!("MCC Token Program: {}", config.mcc_token_program);

    Ok(())
}
