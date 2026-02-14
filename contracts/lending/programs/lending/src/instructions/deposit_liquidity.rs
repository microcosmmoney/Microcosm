// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.is_active @ LendingError::PoolNotActive,
        constraint = !lending_pool.deposits_paused @ LendingError::OperationPaused
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
        constraint = depositor_asset_account.owner == depositor.key(),
        constraint = depositor_asset_account.mint == lending_pool.asset_mint
    )]
    pub depositor_asset_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = depositor_lp_account.owner == depositor.key(),
        constraint = depositor_lp_account.mint == lending_pool.lp_mint
    )]
    pub depositor_lp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DepositLiquidity>, amount: u64) -> Result<()> {
    require!(
        amount >= MIN_DEPOSIT_AMOUNT,
        LendingError::DepositBelowMinimum
    );

    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;

    pool.accrue_interest(clock.unix_timestamp)?;

    let lp_amount = pool.deposit_to_lp(amount);

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.depositor_asset_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;

    let pool_name = pool.name.clone();
    let pool_seeds = &[
        LENDING_POOL_SEED,
        pool_name.as_bytes(),
        &[pool.bump],
    ];
    let signer_seeds = &[&pool_seeds[..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.lp_mint.to_account_info(),
            to: ctx.accounts.depositor_lp_account.to_account_info(),
            authority: pool.to_account_info(),
        },
        signer_seeds,
    );
    token::mint_to(mint_ctx, lp_amount)?;

    pool.total_deposits = pool.total_deposits
        .checked_add(amount)
        .ok_or(LendingError::MathOverflow)?;
    pool.total_lp_supply = pool.total_lp_supply
        .checked_add(lp_amount)
        .ok_or(LendingError::MathOverflow)?;
    pool.updated_at = clock.unix_timestamp;

    msg!("Deposit successful");
    msg!("Amount: {} USDC", amount);
    msg!("LP minted: {}", lp_amount);
    msg!("New total deposits: {}", pool.total_deposits);

    Ok(())
}
