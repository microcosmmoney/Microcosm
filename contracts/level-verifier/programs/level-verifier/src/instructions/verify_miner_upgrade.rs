// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::{UserProfile, UserLevel, VerifierConfig};

#[derive(Accounts)]
pub struct VerifyMinerUpgrade<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [VERIFIER_CONFIG_SEED],
        bump
    )]
    pub verifier_config: Option<Account<'info, VerifierConfig>>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump = user_profile.bump,
        constraint = user_profile.wallet == user.key() @ LevelVerifierError::NotProfileOwner
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VerifyMinerUpgrade>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.level == UserLevel::Prospect,
        LevelVerifierError::PrerequisiteLevelNotMet
    );

    require!(
        profile.mining_days_in_period >= MINER_REQUIRED_MINING_DAYS,
        LevelVerifierError::InsufficientMiningDays
    );

    let old_level = profile.level.to_u8();

    profile.upgrade_to(UserLevel::Miner, clock.unix_timestamp);

    let new_level = profile.level.to_u8();

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.update_level_stats(old_level, new_level);
        config.updated_at = clock.unix_timestamp;
    }

    msg!("Miner upgrade verified!");
    msg!("User: {}", profile.wallet);
    msg!("Mining days: {}/{}", profile.mining_days_in_period, MINER_REQUIRED_MINING_DAYS);
    msg!("Level: {} → {}",
        UserLevel::from_u8(old_level).unwrap().name(),
        profile.level.name()
    );

    Ok(())
}
