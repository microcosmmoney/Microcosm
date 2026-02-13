use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::{Auction, AuctionConfig, AuctionStatus, Bid, BidStatus};

const MCC_DECIMALS: u8 = 9;

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        seeds = [AUCTION_CONFIG_SEED],
        bump,
        constraint = auction_config.is_initialized @ AuctionError::NotInitialized,
        constraint = !auction_config.is_paused @ AuctionError::Paused
    )]
    pub auction_config: Account<'info, AuctionConfig>,

    #[account(
        mut,
        seeds = [AUCTION_SEED, auction_id.to_le_bytes().as_ref()],
        bump = auction.bump,
        constraint = auction.status == AuctionStatus::Active @ AuctionError::AuctionEnded
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        init_if_needed,
        payer = bidder,
        space = Bid::SPACE,
        seeds = [BID_SEED, auction_id.to_le_bytes().as_ref(), bidder.key().as_ref()],
        bump
    )]
    pub bid: Account<'info, Bid>,

    #[account(
        constraint = mcc_mint.key() == auction_config.mcc_mint @ AuctionError::InvalidArgument
    )]
    pub mcc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = bidder_mcc_account.owner == bidder.key() @ AuctionError::Unauthorized,
        constraint = bidder_mcc_account.mint == auction_config.mcc_mint @ AuctionError::InvalidArgument
    )]
    pub bidder_mcc_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [MCC_ESCROW_SEED, auction_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow_mcc_account: InterfaceAccount<'info, TokenAccount>,

    pub previous_bidder_mcc: Option<UncheckedAccount<'info>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<PlaceBid>,
    auction_id: u64,
    amount: u64,
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let config = &ctx.accounts.auction_config;
    let bid = &mut ctx.accounts.bid;
    let clock = Clock::get()?;

    require!(
        !auction.is_ended(clock.unix_timestamp),
        AuctionError::AuctionEnded
    );

    require!(
        auction.bid_count < MAX_BIDS_PER_AUCTION,
        AuctionError::MaxBidsReached
    );

    let min_bid = auction.min_bid_amount(config.min_bid_increment_percent);
    require!(
        amount >= min_bid,
        AuctionError::BidIncrementInsufficient
    );

    if let Some(buy_now) = auction.buy_now_price {
        if amount >= buy_now {
            msg!("Buy now price reached!");
        }
    }

    require!(
        ctx.accounts.bidder_mcc_account.amount >= amount,
        AuctionError::InsufficientBalance
    );

    let previous_amount = if bid.status == BidStatus::Active && bid.amount > 0 {
        bid.amount
    } else {
        0
    };

    let additional_amount = amount.checked_sub(previous_amount)
        .ok_or(AuctionError::InvalidArgument)?;

    if additional_amount > 0 {
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.bidder_mcc_account.to_account_info(),
                mint: ctx.accounts.mcc_mint.to_account_info(),
                to: ctx.accounts.escrow_mcc_account.to_account_info(),
                authority: ctx.accounts.bidder.to_account_info(),
            },
        );
        token_interface::transfer_checked(transfer_ctx, additional_amount, MCC_DECIMALS)?;
    }

    if let Some(prev_highest) = auction.highest_bidder {
        if prev_highest != ctx.accounts.bidder.key() {
            msg!("Previous highest bidder {} will be refunded", prev_highest);
        }
    }

    bid.auction_id = auction_id;
    bid.bidder = ctx.accounts.bidder.key();
    bid.amount = amount;
    bid.status = BidStatus::Active;
    bid.placed_at = clock.unix_timestamp;
    bid.withdrawn_at = None;
    bid.is_refunded = false;
    bid.bump = ctx.bumps.bid;

    auction.update_highest_bid(ctx.accounts.bidder.key(), amount);

    if auction.should_extend(clock.unix_timestamp, config.extension_threshold) {
        auction.extend(config.extension_duration);
        msg!("Auction extended! New end time: {}", auction.end_time);
    }

    if let Some(buy_now) = auction.buy_now_price {
        if amount >= buy_now {
            auction.end_time = clock.unix_timestamp;
            msg!("Buy now triggered! Auction ending immediately.");
        }
    }

    msg!("Bid placed successfully");
    msg!("Bidder: {}", ctx.accounts.bidder.key());
    msg!("Amount: {} MCC", amount);
    msg!("Current highest: {} MCC", auction.current_price);

    Ok(())
}
