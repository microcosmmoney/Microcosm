// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, Burn};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(mut)]
    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.is_active @ LendingError::PoolNotActive
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        constraint = vault.key() == lending_pool.vault @ LendingError::InvalidParameter
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lp_mint.key() == lending_pool.lp_mint @ LendingError::InvalidParameter
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = withdrawer_asset_account.owner == withdrawer.key(),
        constraint = withdrawer_asset_account.mint == lending_pool.asset_mint
    )]
    pub withdrawer_asset_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = withdrawer_lp_account.owner == withdrawer.key(),
        constraint = withdrawer_lp_account.mint == lending_pool.lp_mint
    )]
    pub withdrawer_lp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawLiquidity>, lp_amount: u64) -> Result<()> {
    require!(lp_amount > 0, LendingError::InvalidParameter);
    require!(
        ctx.accounts.withdrawer_lp_account.amount >= lp_amount,
        LendingError::InsufficientLpBalance
    );

    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;

    pool.accrue_interest(clock.unix_timestamp)?;

    msg!("=== Withdraw Debug ===");
    msg!("total_deposits: {}", pool.total_deposits);
    msg!("total_borrowed: {}", pool.total_borrowed);
    msg!("total_lp_supply: {}", pool.total_lp_supply);
    msg!("accrued_interest: {}", pool.accrued_interest);
    msg!("protocol_fees: {}", pool.protocol_fees);

    let asset_amount = pool.lp_to_asset(lp_amount);
    msg!("lp_amount: {}", lp_amount);
    msg!("asset_amount (lp_to_asset): {}", asset_amount);

    let available = pool.available_liquidity();
    msg!("available_liquidity: {}", available);
    msg!("asset_amount <= available: {}", asset_amount <= available);

    require!(
        asset_amount <= available,
        LendingError::InsufficientLiquidity
    );

    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.lp_mint.to_account_info(),
            from: ctx.accounts.withdrawer_lp_account.to_account_info(),
            authority: ctx.accounts.withdrawer.to_account_info(),
        },
    );
    token::burn(burn_ctx, lp_amount)?;

    let pool_name = pool.name.clone();
    let pool_seeds = &[
        LENDING_POOL_SEED,
        pool_name.as_bytes(),
        &[pool.bump],
    ];
    let signer_seeds = &[&pool_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.withdrawer_asset_account.to_account_info(),
            authority: pool.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, asset_amount)?;

    pool.total_deposits = pool.total_deposits.saturating_sub(asset_amount);
    pool.total_lp_supply = pool.total_lp_supply
        .checked_sub(lp_amount)
        .ok_or(LendingError::MathUnderflow)?;
    pool.updated_at = clock.unix_timestamp;

    msg!("Withdrawal successful");
    msg!("LP burned: {}", lp_amount);
    msg!("Amount withdrawn: {} MCC", asset_amount);
    msg!("New total deposits: {}", pool.total_deposits);

    Ok(())
}
