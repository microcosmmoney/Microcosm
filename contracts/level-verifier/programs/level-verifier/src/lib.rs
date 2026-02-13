use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("8TSuNuR1Carh8GpjqrZiVo3oz1nC2K9FSy6BhqbWb3ek");

#[program]
pub mod level_verifier {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        territory_nft_program: Pubkey,
        mcc_token_program: Pubkey,
    ) -> Result<()> {
        instructions::initialize_config::handler(ctx, territory_nft_program, mcc_token_program)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        territory_nft_program: Option<Pubkey>,
        mcc_token_program: Option<Pubkey>,
    ) -> Result<()> {
        instructions::update_config::handler(ctx, territory_nft_program, mcc_token_program)
    }

    pub fn initialize_user_profile(
        ctx: Context<InitializeUserProfile>,
        firebase_uid: String,
    ) -> Result<()> {
        instructions::initialize_user_profile::handler(ctx, firebase_uid)
    }

    pub fn bind_wallet(ctx: Context<BindWallet>) -> Result<()> {
        instructions::bind_wallet::handler(ctx)
    }

    pub fn record_mining_day(ctx: Context<RecordMiningDay>) -> Result<()> {
        instructions::record_mining_day::handler(ctx)
    }

    pub fn verify_miner_upgrade(ctx: Context<VerifyMinerUpgrade>) -> Result<()> {
        instructions::verify_miner_upgrade::handler(ctx)
    }

    pub fn verify_commander_upgrade(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
        instructions::verify_nft_upgrade::verify_commander(ctx)
    }

    pub fn verify_pioneer_upgrade(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
        instructions::verify_nft_upgrade::verify_pioneer(ctx)
    }

    pub fn verify_warden_upgrade(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
        instructions::verify_nft_upgrade::verify_warden(ctx)
    }

    pub fn verify_admiral_upgrade(ctx: Context<VerifyNftUpgrade>) -> Result<()> {
        instructions::verify_nft_upgrade::verify_admiral(ctx)
    }

    pub fn check_level_demotion(ctx: Context<CheckLevelDemotion>) -> Result<()> {
        instructions::check_level_demotion::handler(ctx)
    }
}
