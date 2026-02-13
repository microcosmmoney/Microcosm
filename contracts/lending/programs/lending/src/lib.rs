use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("5JRnecaAZNfF1bBjdW93JB8VFJVpWWDkR3NKg2jHnMPQ");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize_lending_pool(
        ctx: Context<InitializeLendingPool>,
        pool_name: String,
        base_rate: u64,
        optimal_utilization: u64,
        slope1: u64,
        slope2: u64,
    ) -> Result<()> {
        instructions::initialize_lending_pool::handler(
            ctx,
            pool_name,
            base_rate,
            optimal_utilization,
            slope1,
            slope2,
        )
    }

    pub fn initialize_lp_mint(
        ctx: Context<InitializeLpMint>,
    ) -> Result<()> {
        instructions::initialize_lp_mint::handler(ctx)
    }

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
    ) -> Result<()> {
        instructions::initialize_vault::handler(ctx)
    }

    pub fn initialize_nft_oracle(
        ctx: Context<InitializeNftOracle>,
    ) -> Result<()> {
        instructions::initialize_nft_oracle::handler(ctx)
    }

    pub fn deposit_liquidity(
        ctx: Context<DepositLiquidity>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_liquidity::handler(ctx, amount)
    }

    pub fn withdraw_liquidity(
        ctx: Context<WithdrawLiquidity>,
        lp_amount: u64,
    ) -> Result<()> {
        instructions::withdraw_liquidity::handler(ctx, lp_amount)
    }

    pub fn create_loan(
        ctx: Context<CreateLoan>,
        loan_amount: u64,
        duration_type: u8,
    ) -> Result<()> {
        instructions::create_loan::handler(ctx, loan_amount, duration_type)
    }

    pub fn create_loan_init(
        ctx: Context<CreateLoanInit>,
        loan_amount: u64,
        duration_type: u8,
    ) -> Result<()> {
        instructions::create_loan_init::handler(ctx, loan_amount, duration_type)
    }

    pub fn create_loan_escrow(
        ctx: Context<CreateLoanEscrow>,
    ) -> Result<()> {
        instructions::create_loan_escrow::create_loan_escrow(ctx)
    }

    pub fn create_loan_execute(
        ctx: Context<CreateLoanExecute>,
    ) -> Result<()> {
        instructions::create_loan_execute::handler(ctx)
    }

    pub fn repay_loan(
        ctx: Context<RepayLoan>,
        amount: u64,
    ) -> Result<()> {
        instructions::repay_loan::handler(ctx, amount)
    }

    pub fn extend_loan(
        ctx: Context<ExtendLoan>,
        new_duration_type: u8,
    ) -> Result<()> {
        instructions::extend_loan::handler(ctx, new_duration_type)
    }

    pub fn liquidate_loan(
        ctx: Context<LiquidateLoan>,
    ) -> Result<()> {
        instructions::liquidate_loan::handler(ctx)
    }

    pub fn update_pool_params(
        ctx: Context<UpdatePoolParams>,
        new_base_rate: Option<u64>,
        new_optimal_utilization: Option<u64>,
        new_slope1: Option<u64>,
        new_slope2: Option<u64>,
    ) -> Result<()> {
        instructions::update_pool_params::handler(
            ctx,
            new_base_rate,
            new_optimal_utilization,
            new_slope1,
            new_slope2,
        )
    }

    pub fn update_nft_price(
        ctx: Context<UpdateNftPrice>,
        territory_type: u8,
        price_usdc: u64,
    ) -> Result<()> {
        instructions::update_nft_price::handler(ctx, territory_type, price_usdc)
    }

    pub fn update_asset_mint(ctx: Context<UpdateAssetMint>) -> Result<()> {
        instructions::update_asset_mint::handler(ctx)
    }

    pub fn pause_deposits(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::pause_deposits(ctx)
    }

    pub fn unpause_deposits(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::unpause_deposits(ctx)
    }

    pub fn pause_borrows(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::pause_borrows(ctx)
    }

    pub fn unpause_borrows(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::unpause_borrows(ctx)
    }

    pub fn activate_pool(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::activate_pool(ctx)
    }

    pub fn deactivate_pool(ctx: Context<PausePool>) -> Result<()> {
        instructions::pause_pool::deactivate_pool(ctx)
    }

    pub fn sync_pool_state(ctx: Context<SyncPoolState>, force_reset_borrowed: bool) -> Result<()> {
        instructions::sync_pool_state::handler(ctx, force_reset_borrowed)
    }

    pub fn debug_pool(ctx: Context<DebugPool>) -> Result<()> {
        instructions::debug_pool::handler(ctx)
    }
}
