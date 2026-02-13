pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("mCCUkDxoDfnVjTQjQHWkVi1PWBU2jxfQGJUhhDbeq5x");

#[program]
pub mod mcc_token {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        decimals: u8,
    ) -> Result<()> {
        instructions::initialize_token::handler(ctx, decimals)
    }

    pub fn initialize_mining_config(
        ctx: Context<InitializeMiningConfig>,
        initial_price: u64,
    ) -> Result<()> {
        instructions::initialize_mining_config::handler(ctx, initial_price)
    }

    pub fn process_mining_payment(
        ctx: Context<ProcessMiningPayment>,
        usdc_amount: u64,
    ) -> Result<()> {
        instructions::process_mining_payment::handler(ctx, usdc_amount)
    }

    pub fn update_mcc_price(
        ctx: Context<UpdateDhcPrice>,
        new_price: u64,
    ) -> Result<()> {
        instructions::update_dhc_price::handler(ctx, new_price)
    }

    pub fn set_halt_status(
        ctx: Context<SetHaltStatus>,
        is_halted: bool,
    ) -> Result<()> {
        instructions::set_halt_status::handler(ctx, is_halted)
    }

    pub fn get_mining_stats(
        ctx: Context<GetMiningStats>,
    ) -> Result<()> {
        instructions::get_mining_stats::handler(ctx)
    }

    pub fn admin_mining_reward(
        ctx: Context<AdminMiningReward>,
        usdc_equivalent: u64,
    ) -> Result<()> {
        instructions::admin_mining_reward::handler(ctx, usdc_equivalent)
    }

    pub fn initialize_vaults(ctx: Context<InitializeVaults>) -> Result<()> {
        instructions::initialize_vaults::handler(ctx)
    }

    pub fn cycle(
        ctx: Context<Cycle>,
        mcc_amount: u64,
        is_registered: bool,
        min_usdc_out: u64,
    ) -> Result<()> {
        instructions::cycle::handler(ctx, mcc_amount, is_registered, min_usdc_out)
    }

    pub fn dao_create_init(
        ctx: Context<DaoCreateInit>,
        fundraise_id: u64,
        station_id: u64,
        target_amount: u64,
        min_contribution: u64,
        max_contribution: u64,
        deadline_days: u64,
    ) -> Result<()> {
        instructions::dao_create::handler_init(
            ctx,
            fundraise_id,
            station_id,
            target_amount,
            min_contribution,
            max_contribution,
            deadline_days,
        )
    }

    pub fn dao_create_pool(
        ctx: Context<DaoCreatePool>,
        fundraise_id: u64,
    ) -> Result<()> {
        instructions::dao_create::handler_pool(ctx, fundraise_id)
    }

    #[deprecated(since = "2026-01-27", note = "Use dao_create_init + dao_create_pool")]
    pub fn dao_create(
        ctx: Context<DaoCreate>,
        fundraise_id: u64,
        station_id: u64,
        target_amount: u64,
        min_contribution: u64,
        max_contribution: u64,
        deadline_days: u64,
    ) -> Result<()> {
        instructions::dao_create::handler(
            ctx,
            fundraise_id,
            station_id,
            target_amount,
            min_contribution,
            max_contribution,
            deadline_days,
        )
    }

    pub fn dao_contribute(
        ctx: Context<DaoContribute>,
        fundraise_id: u64,
        contributor_uid: u64,
        mcc_amount: u64,
    ) -> Result<()> {
        instructions::dao_contribute::handler(ctx, fundraise_id, contributor_uid, mcc_amount)
    }

    pub fn dao_settle(
        ctx: Context<DaoSettle>,
        fundraise_id: u64,
    ) -> Result<()> {
        instructions::dao_settle::handler_settle(ctx, fundraise_id)
    }

    pub fn dao_refund(
        ctx: Context<DaoRefund>,
        fundraise_id: u64,
        contributor_uid: u64,
    ) -> Result<()> {
        instructions::dao_settle::handler_refund(ctx, fundraise_id, contributor_uid)
    }
}
