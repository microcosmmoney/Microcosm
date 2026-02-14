// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::AuctionConfig;

#[derive(Accounts)]
pub struct InitializeAuctionConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub team_wallet: UncheckedAccount<'info>,

    pub mcc_mint: InterfaceAccount<'info, Mint>,

    pub territory_nft_program: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        space = AuctionConfig::SPACE,
        seeds = [AUCTION_CONFIG_SEED],
        bump
    )]
    pub auction_config: Account<'info, AuctionConfig>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeAuctionConfig>,
    min_bid_increment_percent: u8,
    auction_duration: i64,
    extension_duration: i64,
    extension_threshold: i64,
) -> Result<()> {
    require!(
        min_bid_increment_percent >= MIN_BID_INCREMENT_PERCENT
            && min_bid_increment_percent <= MAX_BID_INCREMENT_PERCENT,
        AuctionError::InvalidBidIncrementPercent
    );

    require!(
        auction_duration > 0 && auction_duration <= 30 * 24 * 60 * 60,
        AuctionError::InvalidAuctionDuration
    );

    require!(
        extension_duration > 0 && extension_duration <= 60 * 60,
        AuctionError::InvalidArgument
    );

    require!(
        extension_threshold > 0 && extension_threshold <= auction_duration,
        AuctionError::InvalidArgument
    );

    let config = &mut ctx.accounts.auction_config;
    let clock = Clock::get()?;

    config.authority = ctx.accounts.authority.key();
    config.team_wallet = ctx.accounts.team_wallet.key();
    config.mcc_mint = ctx.accounts.mcc_mint.key();
    config.territory_nft_program = ctx.accounts.territory_nft_program.key();
    config.min_bid_increment_percent = min_bid_increment_percent;
    config.auction_duration = auction_duration;
    config.extension_duration = extension_duration;
    config.extension_threshold = extension_threshold;
    config.next_auction_id = 1;
    config.total_auctions = 0;
    config.total_completed = 0;
    config.total_cancelled = 0;
    config.total_volume = 0;
    config.is_paused = false;
    config.is_initialized = true;
    config.created_at = clock.unix_timestamp;
    config.updated_at = clock.unix_timestamp;

    msg!("Auction config initialized successfully");
    msg!("Authority: {}", config.authority);
    msg!("Team wallet: {}", config.team_wallet);
    msg!("MCC mint: {}", config.mcc_mint);

    Ok(())
}
