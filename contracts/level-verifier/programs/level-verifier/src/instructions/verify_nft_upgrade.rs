// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::{UserProfile, UserLevel, VerifierConfig};

#[derive(Accounts)]
pub struct VerifyNftUpgrade<'info> {
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

pub fn verify_commander(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.level.to_u8() >= LEVEL_MINER,
        LevelVerifierError::PrerequisiteLevelNotMet
    );

    require!(
        profile.level.to_u8() < LEVEL_COMMANDER,
        LevelVerifierError::AlreadyAtLevel
    );

    require!(
        profile.station_nft_count >= COMMANDER_REQUIRED_STATIONS as u16,
        LevelVerifierError::InsufficientNftCount
    );

    let old_level = profile.level.to_u8();

    profile.upgrade_to(UserLevel::Commander, clock.unix_timestamp);

    let new_level = profile.level.to_u8();

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.update_level_stats(old_level, new_level);
        config.updated_at = clock.unix_timestamp;
    }

    msg!("Commander upgrade verified!");
    msg!("User: {}", profile.wallet);
    msg!("Station NFTs: {}", profile.station_nft_count);

    Ok(())
}

pub fn verify_pioneer(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.level.to_u8() >= LEVEL_COMMANDER,
        LevelVerifierError::PrerequisiteLevelNotMet
    );

    require!(
        profile.level.to_u8() < LEVEL_PIONEER,
        LevelVerifierError::AlreadyAtLevel
    );

    require!(
        profile.station_nft_count >= PIONEER_REQUIRED_STATIONS as u16,
        LevelVerifierError::InsufficientNftCount
    );

    let old_level = profile.level.to_u8();

    profile.upgrade_to(UserLevel::Pioneer, clock.unix_timestamp);

    let new_level = profile.level.to_u8();

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.update_level_stats(old_level, new_level);
        config.updated_at = clock.unix_timestamp;
    }

    msg!("Pioneer upgrade verified!");
    msg!("User: {}", profile.wallet);
    msg!("Station NFTs: {} (required: {})",
        profile.station_nft_count,
        PIONEER_REQUIRED_STATIONS
    );

    Ok(())
}

pub fn verify_warden(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.level.to_u8() >= LEVEL_PIONEER,
        LevelVerifierError::PrerequisiteLevelNotMet
    );

    require!(
        profile.level.to_u8() < LEVEL_WARDEN,
        LevelVerifierError::AlreadyAtLevel
    );

    require!(
        profile.matrix_nft_count >= WARDEN_REQUIRED_MATRICES as u16,
        LevelVerifierError::InsufficientNftCount
    );

    let old_level = profile.level.to_u8();

    profile.upgrade_to(UserLevel::Warden, clock.unix_timestamp);

    let new_level = profile.level.to_u8();

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.update_level_stats(old_level, new_level);
        config.updated_at = clock.unix_timestamp;
    }

    msg!("Warden upgrade verified!");
    msg!("User: {}", profile.wallet);
    msg!("Matrix NFTs: {} (required: {})",
        profile.matrix_nft_count,
        WARDEN_REQUIRED_MATRICES
    );

    Ok(())
}

pub fn verify_admiral(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.level.to_u8() >= LEVEL_WARDEN,
        LevelVerifierError::PrerequisiteLevelNotMet
    );

    require!(
        profile.level.to_u8() < LEVEL_ADMIRAL,
        LevelVerifierError::AlreadyAtLevel
    );

    require!(
        profile.sector_nft_count >= ADMIRAL_REQUIRED_SECTORS as u16,
        LevelVerifierError::InsufficientNftCount
    );

    let old_level = profile.level.to_u8();

    profile.upgrade_to(UserLevel::Admiral, clock.unix_timestamp);

    let new_level = profile.level.to_u8();

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.update_level_stats(old_level, new_level);
        config.updated_at = clock.unix_timestamp;
    }

    msg!("Admiral upgrade verified!");
    msg!("User: {}", profile.wallet);
    msg!("Sector NFTs: {} (required: {})",
        profile.sector_nft_count,
        ADMIRAL_REQUIRED_SECTORS
    );

    Ok(())
}
