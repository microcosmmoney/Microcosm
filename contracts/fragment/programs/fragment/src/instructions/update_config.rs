use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::FragmentConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump,
        constraint = config.authority == authority.key() @ FragmentError::NotAuthority
    )]
    pub config: Account<'info, FragmentConfig>,
}

pub fn handler(
    ctx: Context<UpdateConfig>,
    new_authority: Option<Pubkey>,
    new_min_fragments: Option<u64>,
    new_max_fragments: Option<u64>,
    new_buyout_duration: Option<i64>,
    is_paused: Option<bool>,
) -> Result<()> {
    let clock = Clock::get()?;
    let config = &mut ctx.accounts.config;

    if let Some(authority) = new_authority {
        msg!("Authority updated: {} -> {}", config.authority, authority);
        config.authority = authority;
    }

    if let Some(min_fragments) = new_min_fragments {
        require!(min_fragments >= 2, FragmentError::InvalidMinFragments);
        require!(
            min_fragments <= config.max_fragments,
            FragmentError::MinExceedsMax
        );
        msg!("Min fragments updated: {} -> {}", config.min_fragments, min_fragments);
        config.min_fragments = min_fragments;
    }

    if let Some(max_fragments) = new_max_fragments {
        require!(max_fragments <= MAX_FRAGMENT_COUNT, FragmentError::InvalidMaxFragments);
        require!(
            max_fragments >= config.min_fragments,
            FragmentError::MaxBelowMin
        );
        msg!("Max fragments updated: {} -> {}", config.max_fragments, max_fragments);
        config.max_fragments = max_fragments;
    }

    if let Some(buyout_duration) = new_buyout_duration {
        require!(
            buyout_duration >= MIN_BUYOUT_DURATION,
            FragmentError::BuyoutDurationTooShort
        );
        require!(
            buyout_duration <= MAX_BUYOUT_DURATION,
            FragmentError::BuyoutDurationTooLong
        );
        msg!("Buyout duration updated: {} -> {} seconds", config.buyout_duration, buyout_duration);
        config.buyout_duration = buyout_duration;
    }

    if let Some(paused) = is_paused {
        msg!("Paused status updated: {} -> {}", config.is_paused, paused);
        config.is_paused = paused;
    }

    config.updated_at = clock.unix_timestamp;

    msg!("Configuration updated successfully!");

    Ok(())
}
