use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("REn8oKyydvjRsistZ2cVi6tksPubvR3bEuLdVTyGknb");

#[program]
pub mod reincarnation {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        pool_name: String,
        base_price: u64,
        premium_bps: u64,
        daily_limit: u64,
    ) -> Result<()> {
        instructions::initialize_pool::handler(
            ctx,
            pool_name,
            base_price,
            premium_bps,
            daily_limit,
        )
    }

    pub fn initialize_vaults(ctx: Context<InitializeVaults>) -> Result<()> {
        instructions::initialize_vaults::handler(ctx)
    }

    pub fn initialize_mcc_vault(ctx: Context<InitializeMccVault>) -> Result<()> {
        instructions::initialize_vaults::handler_mcc_vault(ctx)
    }

    pub fn initialize_mcd_vault(ctx: Context<InitializeMcdVault>) -> Result<()> {
        instructions::initialize_vaults::handler_mcd_vault(ctx)
    }

    pub fn initialize_usdt_vault(ctx: Context<InitializeUsdtVault>) -> Result<()> {
        instructions::initialize_usdt_vault::handler(ctx)
    }

    pub fn execute_buyback(
        ctx: Context<ExecuteBuyback>,
        mcc_amount: u64,
    ) -> Result<()> {
        instructions::execute_buyback::handler(ctx, mcc_amount)
    }

    pub fn execute_mining(
        ctx: Context<ExecuteMining>,
        mcc_amount: u64,
        usdc_amount: u64,
    ) -> Result<()> {
        instructions::execute_mining::handler(ctx, mcc_amount, usdc_amount)
    }

    pub fn execute_monthly_cycle(ctx: Context<ExecuteMonthlyCycle>) -> Result<()> {
        instructions::execute_monthly_cycle::handler(ctx)
    }

    pub fn update_price(
        ctx: Context<UpdatePrice>,
        new_base_price: u64,
    ) -> Result<()> {
        instructions::update_price::handler(ctx, new_base_price)
    }

    pub fn withdraw_usdc(
        ctx: Context<WithdrawUsdc>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw_usdc::handler(ctx, amount)
    }

    pub fn pause_pool(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::handler_pause(ctx)
    }

    pub fn unpause_pool(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::handler_unpause(ctx)
    }

    pub fn close_pool(ctx: Context<ClosePool>) -> Result<()> {
        instructions::close_pool::handler(ctx)
    }

    pub fn set_vaults(ctx: Context<SetVaults>) -> Result<()> {
        instructions::set_vaults::handler(ctx)
    }

    pub fn migrate_pool(ctx: Context<MigratePool>) -> Result<()> {
        instructions::migrate_pool::handler(ctx)
    }

    pub fn set_mcd_fields(
        ctx: Context<SetMcdFields>,
        mcd_mint: Pubkey,
        mcd_vault: Pubkey,
    ) -> Result<()> {
        instructions::set_mcd_fields::handler(ctx, mcd_mint, mcd_vault)
    }

    pub fn set_usdt_fields(
        ctx: Context<SetUsdtFields>,
        usdt_mint: Pubkey,
        usdt_vault: Pubkey,
    ) -> Result<()> {
        instructions::set_usdt_fields::handler(ctx, usdt_mint, usdt_vault)
    }

    pub fn fix_bump(ctx: Context<FixBump>) -> Result<()> {
        instructions::fix_bump::handler(ctx)
    }

    pub fn fix_pool_data(
        ctx: Context<FixPoolData>,
        usdt_mint: Pubkey,
        usdt_vault: Pubkey,
        base_price: u64,
        premium_bps: u64,
        daily_limit: u64,
    ) -> Result<()> {
        instructions::fix_pool_data::handler(ctx, usdt_mint, usdt_vault, base_price, premium_bps, daily_limit)
    }

    pub fn set_token_mints(
        ctx: Context<SetTokenMints>,
        mcc_mint: Pubkey,
        mcd_mint: Pubkey,
    ) -> Result<()> {
        instructions::set_token_mints::handler(ctx, mcc_mint, mcd_mint)
    }

    pub fn set_vault_addresses(
        ctx: Context<SetVaultAddresses>,
        mcc_vault: Pubkey,
        mcd_vault: Pubkey,
    ) -> Result<()> {
        instructions::set_vault_addresses::handler(ctx, mcc_vault, mcd_vault)
    }

    pub fn close_old_vault_ata(ctx: Context<CloseOldVaultAta>) -> Result<()> {
        instructions::close_old_vault_ata::handler(ctx)
    }
}
