use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("J7wyftDtfzqn2pbpLr8QPqKQkAxecgVze75SUz9D4Fq3");

#[program]
pub mod fragment {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::initialize_config::handler(ctx)
    }

    pub fn fragmentize_nft(
        ctx: Context<FragmentizeNft>,
        fragment_count: u64,
        fragment_name: String,
        fragment_symbol: String,
    ) -> Result<()> {
        instructions::fragmentize_nft::handler(
            ctx,
            fragment_count,
            fragment_name,
            fragment_symbol,
        )
    }

    pub fn fragmentize_nft_init_vault(
        ctx: Context<FragmentizeNftInitVault>,
        fragment_count: u64,
        fragment_name: String,
        fragment_symbol: String,
    ) -> Result<()> {
        instructions::fragmentize_nft_v2::handler_init_vault(
            ctx,
            fragment_count,
            fragment_name,
            fragment_symbol,
        )
    }

    pub fn fragmentize_nft_init_token(ctx: Context<FragmentizeNftInitToken>) -> Result<()> {
        instructions::fragmentize_nft_v2::handler_init_token(ctx)
    }

    pub fn fragmentize_nft_finalize(ctx: Context<FragmentizeNftFinalize>) -> Result<()> {
        instructions::fragmentize_nft_v2::handler_finalize(ctx)
    }

    pub fn redeem_nft(ctx: Context<RedeemNft>) -> Result<()> {
        instructions::redeem_nft::handler(ctx)
    }

    pub fn redeem_nft_burn_fragments(ctx: Context<RedeemNftBurnFragments>) -> Result<()> {
        instructions::redeem_nft_v2::handler_burn_fragments(ctx)
    }

    pub fn redeem_nft_finalize(ctx: Context<RedeemNftFinalize>) -> Result<()> {
        instructions::redeem_nft_v2::handler_finalize(ctx)
    }

    pub fn transfer_fragments(
        ctx: Context<TransferFragments>,
        amount: u64,
    ) -> Result<()> {
        instructions::transfer_fragments::handler(ctx, amount)
    }

    pub fn initiate_buyout(
        ctx: Context<InitiateBuyout>,
        price_per_fragment: u64,
    ) -> Result<()> {
        instructions::initiate_buyout::handler(ctx, price_per_fragment)
    }

    pub fn initiate_buyout_init(
        ctx: Context<InitiateBuyoutInit>,
        price_per_fragment: u64,
    ) -> Result<()> {
        instructions::initiate_buyout_v2::handler_init(ctx, price_per_fragment)
    }

    pub fn initiate_buyout_escrow(ctx: Context<InitiateBuyoutEscrow>) -> Result<()> {
        instructions::initiate_buyout_v2::handler_escrow(ctx)
    }

    pub fn accept_buyout(
        ctx: Context<AcceptBuyout>,
        fragment_amount: u64,
    ) -> Result<()> {
        instructions::accept_buyout::handler(ctx, fragment_amount)
    }

    pub fn complete_buyout(ctx: Context<CompleteBuyout>) -> Result<()> {
        instructions::complete_buyout::handler(ctx)
    }

    pub fn cancel_buyout(ctx: Context<CancelBuyout>) -> Result<()> {
        instructions::cancel_buyout::handler(ctx)
    }

    pub fn cancel_buyout_refund(ctx: Context<CancelBuyoutRefund>) -> Result<()> {
        instructions::cancel_buyout_v2::handler_refund(ctx)
    }

    pub fn cancel_buyout_close(ctx: Context<CancelBuyoutClose>) -> Result<()> {
        instructions::cancel_buyout_v2::handler_close(ctx)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_authority: Option<Pubkey>,
        new_min_fragments: Option<u64>,
        new_max_fragments: Option<u64>,
        new_buyout_duration: Option<i64>,
        is_paused: Option<bool>,
    ) -> Result<()> {
        instructions::update_config::handler(
            ctx,
            new_authority,
            new_min_fragments,
            new_max_fragments,
            new_buyout_duration,
            is_paused,
        )
    }

    pub fn close_redeemed_vault(ctx: Context<CloseRedeemedVault>) -> Result<()> {
        instructions::close_redeemed_vault::handler(ctx)
    }
}
