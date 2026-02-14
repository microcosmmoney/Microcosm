// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::{UserProfile, UserLevel, VerifierConfig};

#[derive(Accounts)]
#[instruction(firebase_uid: String)]
pub struct InitializeUserProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [VERIFIER_CONFIG_SEED],
        bump
    )]
    pub verifier_config: Option<Account<'info, VerifierConfig>>,

    #[account(
        init,
        payer = user,
        space = UserProfile::SPACE,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeUserProfile>,
    firebase_uid: String,
) -> Result<()> {
    require!(
        !firebase_uid.is_empty() && firebase_uid.len() <= MAX_FIREBASE_UID_LENGTH,
        LevelVerifierError::InvalidFirebaseUid
    );

    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    profile.wallet = ctx.accounts.user.key();
    profile.firebase_uid = firebase_uid.clone();
    profile.level = UserLevel::Recruit;
    profile.wallet_bound_at = None;
    profile.first_mining_at = None;
    profile.last_mining_at = None;
    profile.mining_days_in_period = 0;
    profile.mining_bitmap = 0;
    profile.mining_bitmap_start_day = 0;
    profile.station_nft_count = 0;
    profile.matrix_nft_count = 0;
    profile.sector_nft_count = 0;
    profile.system_nft_count = 0;
    profile.level_upgraded_at = None;
    profile.created_at = clock.unix_timestamp;
    profile.updated_at = clock.unix_timestamp;
    profile.bump = ctx.bumps.user_profile;

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.increment_users();
        config.updated_at = clock.unix_timestamp;
    }

    msg!("User profile initialized");
    msg!("Wallet: {}", profile.wallet);
    msg!("Firebase UID: {}", firebase_uid);
    msg!("Level: {} ({})", profile.level.to_u8(), profile.level.name());

    Ok(())
}
