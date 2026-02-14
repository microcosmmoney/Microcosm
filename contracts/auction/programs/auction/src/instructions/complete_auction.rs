// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::{Auction, AuctionConfig, AuctionStatus, AuctionType, Bid, BidStatus};

const MCC_DECIMALS: u8 = 9;

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct CompleteAuction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

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

    #[account(
        mut,
        seeds = [
            BID_SEED,
            auction_id.to_le_bytes().as_ref(),
            auction.highest_bidder.unwrap().as_ref()
        ],
        bump = winner_bid.bump,
        constraint = winner_bid.status == BidStatus::Active @ AuctionError::BidNotFound
    )]
    pub winner_bid: Account<'info, Bid>,

    #[account(
        constraint = mcc_mint.key() == auction_config.mcc_mint @ AuctionError::InvalidArgument
    )]
    pub mcc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [MCC_ESCROW_SEED, auction_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow_mcc_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = proceeds_recipient_mcc.owner == auction.proceeds_recipient @ AuctionError::InvalidArgument
    )]
    pub proceeds_recipient_mcc: InterfaceAccount<'info, TokenAccount>,

    pub winner: UncheckedAccount<'info>,

    pub territory_nft_program: UncheckedAccount<'info>,

    pub collection_config: UncheckedAccount<'info>,

    pub territory_nft: UncheckedAccount<'info>,

    pub nft_mint: UncheckedAccount<'info>,

    pub nft_metadata: UncheckedAccount<'info>,

    pub nft_master_edition: UncheckedAccount<'info>,

    pub winner_nft_ata: UncheckedAccount<'info>,

    pub collection_mint: UncheckedAccount<'info>,

    pub collection_metadata: UncheckedAccount<'info>,

    pub collection_master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CompleteAuction>,
    auction_id: u64,
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let config = &mut ctx.accounts.auction_config;
    let winner_bid = &mut ctx.accounts.winner_bid;
    let clock = Clock::get()?;

    require!(
        auction.is_ended(clock.unix_timestamp),
        AuctionError::AuctionNotEnded
    );

    require!(
        auction.highest_bidder.is_some(),
        AuctionError::BidNotFound
    );

    let winner = auction.highest_bidder.unwrap();
    require!(
        winner == ctx.accounts.winner.key(),
        AuctionError::InvalidArgument
    );

    let winning_amount = auction.current_price;

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
            to: ctx.accounts.proceeds_recipient_mcc.to_account_info(),
            authority: ctx.accounts.escrow_mcc_account.to_account_info(),
        },
        escrow_signer,
    );
    token_interface::transfer_checked(transfer_ctx, winning_amount, MCC_DECIMALS)?;

    match auction.auction_type {
        AuctionType::First => {
            msg!("First auction: Minting new NFT to winner");
            msg!("Territory: {:?} #{}", auction.territory_type, auction.territory_id);

        }
        AuctionType::Secondary => {
            msg!("Secondary auction: Transferring NFT to winner");

        }
    }

    winner_bid.mark_won();

    auction.complete(clock.unix_timestamp);

    config.record_completion(winning_amount);
    config.updated_at = clock.unix_timestamp;

    msg!("Auction completed successfully");
    msg!("Auction ID: {}", auction_id);
    msg!("Winner: {}", winner);
    msg!("Winning bid: {} MCC", winning_amount);
    msg!("Proceeds recipient: {}", auction.proceeds_recipient);

    Ok(())
}
