use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::{Auction, AuctionConfig, AuctionStatus, Bid, BidStatus};

const MCC_DECIMALS: u8 = 9;

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct CancelBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        seeds = [AUCTION_CONFIG_SEED],
        bump
    )]
    pub auction_config: Account<'info, AuctionConfig>,

    #[account(
        seeds = [AUCTION_SEED, auction_id.to_le_bytes().as_ref()],
        bump = auction.bump
    )]
    pub auction: Account<'info, Auction>,

    #[account(
        mut,
        seeds = [BID_SEED, auction_id.to_le_bytes().as_ref(), bidder.key().as_ref()],
        bump = bid.bump,
        constraint = bid.bidder == bidder.key() @ AuctionError::NotBidOwner,
        constraint = bid.status == BidStatus::Active @ AuctionError::BidAlreadyCancelled
    )]
    pub bid: Account<'info, Bid>,

    #[account(
        constraint = mcc_mint.key() == auction_config.mcc_mint @ AuctionError::InvalidArgument
    )]
    pub mcc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = bidder_mcc_account.owner == bidder.key() @ AuctionError::Unauthorized
    )]
    pub bidder_mcc_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [MCC_ESCROW_SEED, auction_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow_mcc_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CancelBid>,
    auction_id: u64,
) -> Result<()> {
    let auction = &ctx.accounts.auction;
    let bid = &mut ctx.accounts.bid;
    let clock = Clock::get()?;

    if let Some(highest_bidder) = auction.highest_bidder {
        require!(
            highest_bidder != ctx.accounts.bidder.key(),
            AuctionError::CannotCancelHighestBid
        );
    }

    let can_cancel = match auction.status {
        AuctionStatus::Active => true,
        AuctionStatus::Cancelled => true,
        AuctionStatus::Completed => {
            auction.highest_bidder != Some(ctx.accounts.bidder.key())
        }
        _ => false,
    };
    require!(can_cancel, AuctionError::AuctionEnded);

    let refund_amount = bid.amount;
    require!(refund_amount > 0, AuctionError::InvalidArgument);

    let auction_id_bytes = auction_id.to_le_bytes();
    let escrow_seeds = &[
        MCC_ESCROW_SEED,
        auction_id_bytes.as_ref(),
        &[ctx.bumps.escrow_mcc_account],
    ];
    let escrow_signer = &[&escrow_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.escrow_mcc_account.to_account_info(),
            mint: ctx.accounts.mcc_mint.to_account_info(),
            to: ctx.accounts.bidder_mcc_account.to_account_info(),
            authority: ctx.accounts.escrow_mcc_account.to_account_info(),
        },
        escrow_signer,
    );
    token_interface::transfer_checked(transfer_ctx, refund_amount, MCC_DECIMALS)?;

    bid.withdraw(clock.unix_timestamp);
    bid.mark_refunded();

    msg!("Bid cancelled successfully");
    msg!("Refunded: {} MCC", refund_amount);
    msg!("Bidder: {}", ctx.accounts.bidder.key());

    Ok(())
}
