use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::*;
use crate::error::AuctionError;
use crate::state::{Auction, AuctionConfig, AuctionStatus, AuctionType, TerritoryType};

#[derive(Accounts)]
#[instruction(territory_type: u8, territory_id: u64)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [AUCTION_CONFIG_SEED],
        bump,
        constraint = auction_config.is_initialized @ AuctionError::NotInitialized,
        constraint = !auction_config.is_paused @ AuctionError::Paused
    )]
    pub auction_config: Box<Account<'info, AuctionConfig>>,

    #[account(
        init,
        payer = creator,
        space = Auction::SPACE,
        seeds = [
            AUCTION_SEED,
            auction_config.next_auction_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub auction: Box<Account<'info, Auction>>,

    #[account(
        constraint = mcc_mint.key() == auction_config.mcc_mint @ AuctionError::InvalidArgument
    )]
    pub mcc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        token::mint = mcc_mint,
        token::authority = escrow_mcc_account,
        token::token_program = token_program,
        seeds = [MCC_ESCROW_SEED, auction_config.next_auction_id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow_mcc_account: InterfaceAccount<'info, TokenAccount>,

    pub proceeds_recipient: UncheckedAccount<'info>,

    pub territory_nft: Option<UncheckedAccount<'info>>,

    pub nft_mint: Option<UncheckedAccount<'info>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateAuction>,
    territory_type: u8,
    territory_id: u64,
    starting_price: u64,
    buy_now_price: Option<u64>,
) -> Result<()> {
    let territory_type_enum = TerritoryType::from_u8(territory_type)
        .ok_or(AuctionError::InvalidTerritoryType)?;

    require!(
        starting_price >= MIN_STARTING_PRICE,
        AuctionError::BidBelowStartingPrice
    );

    if let Some(buy_now) = buy_now_price {
        require!(
            buy_now > starting_price,
            AuctionError::InvalidArgument
        );
    }

    let config = &mut ctx.accounts.auction_config;
    let auction = &mut ctx.accounts.auction;
    let clock = Clock::get()?;

    let (auction_type, nft_mint_key) = if ctx.accounts.territory_nft.is_some() {
        let mint = ctx.accounts.nft_mint.as_ref()
            .ok_or(AuctionError::InvalidArgument)?;
        (AuctionType::Secondary, Some(mint.key()))
    } else {
        require!(
            ctx.accounts.creator.key() == config.authority,
            AuctionError::Unauthorized
        );
        (AuctionType::First, None)
    };

    let auction_id = config.generate_auction_id();

    let proceeds_recipient = match auction_type {
        AuctionType::First => config.team_wallet,
        AuctionType::Secondary => ctx.accounts.creator.key(),
    };

    auction.auction_id = auction_id;
    auction.auction_type = auction_type;
    auction.status = AuctionStatus::Active;
    auction.territory_type = territory_type_enum;
    auction.territory_id = territory_id;
    auction.creator = ctx.accounts.creator.key();
    auction.starting_price = starting_price;
    auction.buy_now_price = buy_now_price;
    auction.current_price = 0;
    auction.highest_bidder = None;
    auction.bid_count = 0;
    auction.start_time = clock.unix_timestamp;
    auction.end_time = clock.unix_timestamp
        .checked_add(config.auction_duration)
        .ok_or(AuctionError::Overflow)?;
    auction.extension_count = 0;
    auction.proceeds_recipient = proceeds_recipient;
    auction.settled_amount = 0;
    auction.nft_mint = nft_mint_key;
    auction.created_at = clock.unix_timestamp;
    auction.completed_at = None;
    auction.bump = ctx.bumps.auction;

    config.updated_at = clock.unix_timestamp;

    msg!("Auction created successfully");
    msg!("Auction ID: {}", auction_id);
    msg!("Territory: {:?} #{}", territory_type_enum, territory_id);
    msg!("Type: {:?}", auction_type);
    msg!("Starting price: {} MCC", starting_price);
    msg!("End time: {}", auction.end_time);

    Ok(())
}
