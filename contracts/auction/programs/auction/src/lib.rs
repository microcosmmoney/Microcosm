use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("6JRmShodszZca3ez7GfDkmsRSzTi5ELwscWCDgjsteJx");

#[program]
pub mod auction {
    use super::*;

    pub fn initialize_auction_config(
        ctx: Context<InitializeAuctionConfig>,
        min_bid_increment_percent: u8,
        auction_duration: i64,
        extension_duration: i64,
        extension_threshold: i64,
    ) -> Result<()> {
        instructions::initialize_auction_config::handler(
            ctx,
            min_bid_increment_percent,
            auction_duration,
            extension_duration,
            extension_threshold,
        )
    }

    pub fn update_auction_config(
        ctx: Context<UpdateAuctionConfig>,
        params: UpdateConfigParams,
    ) -> Result<()> {
        instructions::update_auction_config::handler(ctx, params)
    }

    pub fn create_auction(
        ctx: Context<CreateAuction>,
        territory_type: u8,
        territory_id: u64,
        starting_price: u64,
        buy_now_price: Option<u64>,
    ) -> Result<()> {
        instructions::create_auction::handler(
            ctx,
            territory_type,
            territory_id,
            starting_price,
            buy_now_price,
        )
    }

    pub fn place_bid(
        ctx: Context<PlaceBid>,
        auction_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::place_bid::handler(ctx, auction_id, amount)
    }

    pub fn cancel_bid(
        ctx: Context<CancelBid>,
        auction_id: u64,
    ) -> Result<()> {
        instructions::cancel_bid::handler(ctx, auction_id)
    }

    pub fn complete_auction(
        ctx: Context<CompleteAuction>,
        auction_id: u64,
    ) -> Result<()> {
        instructions::complete_auction::handler(ctx, auction_id)
    }

    pub fn cancel_auction(
        ctx: Context<CancelAuction>,
        auction_id: u64,
    ) -> Result<()> {
        instructions::cancel_auction::handler(ctx, auction_id)
    }
}
