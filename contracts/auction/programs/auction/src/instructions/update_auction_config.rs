// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::AuctionConfig;

#[derive(Accounts)]
pub struct UpdateAuctionConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_CONFIG_SEED],
        bump,
        constraint = auction_config.is_initialized @ AuctionError::NotInitialized,
        constraint = auction_config.authority == authority.key() @ AuctionError::Unauthorized
    )]
    pub auction_config: Account<'info, AuctionConfig>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateConfigParams {
    pub auction_duration: Option<i64>,
    pub extension_duration: Option<i64>,
    pub extension_threshold: Option<i64>,
    pub min_bid_increment_percent: Option<u8>,
}

pub fn handler(
    ctx: Context<UpdateAuctionConfig>,
    params: UpdateConfigParams,
) -> Result<()> {
    let config = &mut ctx.accounts.auction_config;
    let clock = Clock::get()?;

    if let Some(duration) = params.auction_duration {
        require!(duration > 0, AuctionError::InvalidArgument);
        msg!("Updating auction_duration: {} -> {}", config.auction_duration, duration);
        config.auction_duration = duration;
    }

    if let Some(extension) = params.extension_duration {
        require!(extension > 0, AuctionError::InvalidArgument);
        msg!("Updating extension_duration: {} -> {}", config.extension_duration, extension);
        config.extension_duration = extension;
    }

    if let Some(threshold) = params.extension_threshold {
        require!(threshold > 0, AuctionError::InvalidArgument);
        msg!("Updating extension_threshold: {} -> {}", config.extension_threshold, threshold);
        config.extension_threshold = threshold;
    }

    if let Some(percent) = params.min_bid_increment_percent {
        require!(
            percent >= MIN_BID_INCREMENT_PERCENT && percent <= MAX_BID_INCREMENT_PERCENT,
            AuctionError::InvalidArgument
        );
        msg!("Updating min_bid_increment_percent: {} -> {}", config.min_bid_increment_percent, percent);
        config.min_bid_increment_percent = percent;
    }

    config.updated_at = clock.unix_timestamp;

    msg!("Auction config updated successfully");

    Ok(())
}
