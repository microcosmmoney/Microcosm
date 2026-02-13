pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("McDVieuEFv5ucnM3C7p8wsT6LY8BAipKDrTG9HjAu3R");

#[program]
pub mod mcd_token {
    use super::*;

    pub fn initialize_mcd(
        ctx: Context<InitializeMcd>,
        decimals: u8,
    ) -> Result<()> {
        instructions::initialize_mcd::handler(ctx, decimals)
    }

    pub fn initialize_mcd_config(
        ctx: Context<InitializeMcdConfig>,
    ) -> Result<()> {
        instructions::initialize_mcd_config::handler(ctx)
    }

    pub fn initialize_station_vault(
        ctx: Context<InitializeStationVault>,
        station_id: u64,
    ) -> Result<()> {
        instructions::initialize_station_vault::handler(ctx, station_id)
    }

    pub fn initialize_user_mcd_account(
        ctx: Context<InitializeUserMcdAccount>,
        uid: u64,
    ) -> Result<()> {
        instructions::initialize_user_mcd_account::handler(ctx, uid)
    }

    pub fn mint_mining_mcd(
        ctx: Context<MintMiningMcd>,
        station_id: u64,
        mcd_amount: u64,
    ) -> Result<()> {
        instructions::mint_mining_mcd::handler(ctx, station_id, mcd_amount)
    }

    pub fn mint_genesis_supply(
        ctx: Context<MintGenesisSupply>,
        amount: u64,
    ) -> Result<()> {
        instructions::mint_genesis_supply::handler(ctx, amount)
    }

    pub fn distribute_daily_mcd<'info>(
        ctx: Context<'_, '_, 'info, 'info, DistributeDailyMcd<'info>>,
        station_id: u64,
        distributions: Vec<McdDistribution>,
    ) -> Result<()> {
        instructions::distribute_daily_mcd::handler(ctx, station_id, distributions)
    }

    pub fn consume_mcd(
        ctx: Context<ConsumeMcd>,
        from_uid: u64,
        to_project_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::consume_mcd::handler(ctx, from_uid, to_project_id, amount)
    }

    pub fn mcd_mining_payment(
        ctx: Context<McdMiningPayment>,
        uid: u64,
        station_id: u64,
        mcd_amount: u64,
    ) -> Result<()> {
        instructions::mcd_mining_payment::handler(ctx, uid, station_id, mcd_amount)
    }

    pub fn migrate_config(ctx: Context<MigrateConfig>) -> Result<()> {
        instructions::migrate_config::handler(ctx)
    }

    pub fn fix_config_bump(ctx: Context<FixConfigBump>) -> Result<()> {
        instructions::fix_config_bump::handler(ctx)
    }

    pub fn add_to_whitelist(
        ctx: Context<AddToWhitelist>,
        project_id: u64,
        project_name: String,
    ) -> Result<()> {
        instructions::add_to_whitelist::handler(ctx, project_id, project_name)
    }

    pub fn remove_from_whitelist(
        ctx: Context<RemoveFromWhitelist>,
        project_id: u64,
    ) -> Result<()> {
        instructions::remove_from_whitelist::handler(ctx, project_id)
    }

    pub fn update_whitelist_status(
        ctx: Context<UpdateWhitelistStatus>,
        project_id: u64,
        status: u8,
    ) -> Result<()> {
        instructions::update_whitelist_status::handler(ctx, project_id, status)
    }

    pub fn close_user_mcd_account(
        ctx: Context<CloseUserMcdAccount>,
        uid: u64,
    ) -> Result<()> {
        instructions::close_user_mcd_account::handler(ctx, uid)
    }

    pub fn close_old_station_vault_ata(
        ctx: Context<CloseOldStationVaultAta>,
        station_id: u64,
    ) -> Result<()> {
        instructions::close_old_station_vault_ata::handler(ctx, station_id)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct McdDistribution {
    pub uid: u64,
    pub amount: u64,
}
