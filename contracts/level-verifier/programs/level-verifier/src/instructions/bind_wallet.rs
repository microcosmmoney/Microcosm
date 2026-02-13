use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::{UserProfile, UserLevel, VerifierConfig};

#[derive(Accounts)]
pub struct BindWallet<'info> {
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

pub fn handler(ctx: Context<BindWallet>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.wallet_bound_at.is_none(),
        LevelVerifierError::WalletAlreadyBound
    );

    require!(
        profile.level == UserLevel::Recruit,
        LevelVerifierError::AlreadyAtLevel
    );

    let old_level = profile.level.to_u8();

    profile.bind_wallet(clock.unix_timestamp);

    let new_level = profile.level.to_u8();

    if let Some(config) = &mut ctx.accounts.verifier_config {
        config.update_level_stats(old_level, new_level);
        config.updated_at = clock.unix_timestamp;
    }

    msg!("Wallet bound successfully");
    msg!("User: {}", profile.wallet);
    msg!("Upgraded: {} → {}",
        UserLevel::from_u8(old_level).unwrap().name(),
        profile.level.name()
    );

    Ok(())
}
