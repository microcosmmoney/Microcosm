use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::{Auction, AuctionConfig, AuctionStatus};

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct CancelAuction<'info> {
    #[account(mut)]
    pub canceller: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_CONFIG_SEED],
        bump
    )]
    pub auction_config: Account<'info, AuctionConfig>,

    #[account(
        mut,
        seeds = [AUCTION_SEED, auction_id.to_le_bytes().as_ref()],
        bump = auction.bump,
        constraint = auction.status == AuctionStatus::Active @ AuctionError::AuctionCompleted
    )]
    pub auction: Account<'info, Auction>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CancelAuction>,
    auction_id: u64,
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let config = &mut ctx.accounts.auction_config;
    let clock = Clock::get()?;

    let is_authority = ctx.accounts.canceller.key() == config.authority;
    let is_creator = ctx.accounts.canceller.key() == auction.creator;
    require!(
        is_authority || is_creator,
        AuctionError::Unauthorized
    );

    if auction.bid_count > 0 {
        require!(
            is_authority,
            AuctionError::AuctionInProgress
        );
        msg!("Auction has {} bids, all will need to be refunded", auction.bid_count);
    }

    auction.cancel();

    config.record_cancellation();
    config.updated_at = clock.unix_timestamp;

    msg!("Auction cancelled successfully");
    msg!("Auction ID: {}", auction_id);
    msg!("Cancelled by: {}", ctx.accounts.canceller.key());
    msg!("Total bids to refund: {}", auction.bid_count);

    Ok(())
}
