// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::{UserProfile, UserLevel, VerifierConfig};

#[derive(Accounts)]
pub struct CheckLevelDemotion<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        mut,
        seeds = [VERIFIER_CONFIG_SEED],
        bump
    )]
    pub verifier_config: Option<Account<'info, VerifierConfig>>,

    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [USER_PROFILE_SEED, user.key().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CheckLevelDemotion>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    let old_level = profile.level.to_u8();

    if let Some(new_level) = profile.check_demotion() {
        let new_level_u8 = new_level.to_u8();

        profile.level = new_level;
        profile.level_upgraded_at = Some(clock.unix_timestamp);
        profile.updated_at = clock.unix_timestamp;

        if let Some(config) = &mut ctx.accounts.verifier_config {
            config.update_level_stats(old_level, new_level_u8);
            config.updated_at = clock.unix_timestamp;
        }

        msg!("Level demotion detected!");
        msg!("User: {}", profile.wallet);
        msg!("Demoted: {} → {}",
            UserLevel::from_u8(old_level).unwrap().name(),
            new_level.name()
        );

        while let Some(further_level) = profile.check_demotion() {
            let current = profile.level.to_u8();
            profile.level = further_level;

            if let Some(config) = &mut ctx.accounts.verifier_config {
                config.update_level_stats(current, further_level.to_u8());
            }

            msg!("Further demotion: {} → {}",
                UserLevel::from_u8(current).unwrap().name(),
                further_level.name()
            );
        }
    } else {
        msg!("No demotion needed");
        msg!("User: {}", profile.wallet);
        msg!("Current level: {}", profile.level.name());
        msg!("Station NFTs: {}", profile.station_nft_count);
        msg!("Matrix NFTs: {}", profile.matrix_nft_count);
        msg!("Sector NFTs: {}", profile.sector_nft_count);
    }

    Ok(())
}
