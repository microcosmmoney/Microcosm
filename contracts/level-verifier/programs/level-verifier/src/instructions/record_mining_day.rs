use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::{UserProfile, UserLevel};

#[derive(Accounts)]
pub struct RecordMiningDay<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RecordMiningDay>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    require!(
        profile.level.to_u8() >= LEVEL_PROSPECT,
        LevelVerifierError::PrerequisiteLevelNotMet
    );

    let recorded = profile.record_mining(clock.unix_timestamp);
    require!(recorded, LevelVerifierError::AlreadyMinedToday);

    profile.updated_at = clock.unix_timestamp;

    msg!("Mining day recorded");
    msg!("User: {}", profile.wallet);
    msg!("Mining days in period: {}/{}",
        profile.mining_days_in_period,
        MINING_PERIOD_DAYS
    );

    if profile.can_upgrade_to_miner() {
        msg!("User qualifies for Miner upgrade!");
    }

    Ok(())
}
