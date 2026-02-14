// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{
    self, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

#[inline(never)]
fn do_transfer_checked<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from,
        to,
        authority,
        mint,
    };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
    token_interface::transfer_checked(cpi_ctx, amount, decimals)
}

#[inline(never)]
fn do_transfer_checked_signed<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from,
        to,
        authority,
        mint,
    };
    let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);
    token_interface::transfer_checked(cpi_ctx, amount, decimals)
}

pub fn handler(
    ctx: Context<ExecuteBuyback>,
    mcc_amount: u64,
) -> Result<()> {
    require!(mcc_amount > 0, ReincarnationError::ZeroAmount);
    require!(
        mcc_amount >= MIN_BUYBACK_AMOUNT,
        ReincarnationError::AmountBelowMinimum
    );
    require!(
        mcc_amount <= MAX_BUYBACK_AMOUNT,
        ReincarnationError::AmountExceedsMaximum
    );

    let clock = Clock::get()?;

    let pool_bump = ctx.accounts.reincarnation_pool.bump;

    {
        let pool = &ctx.accounts.reincarnation_pool;
        require!(!pool.paused, ReincarnationError::PoolPaused);
    }

    let usdc_amount;
    {
        let pool = &mut ctx.accounts.reincarnation_pool;

        pool.check_and_reset_daily_limit(clock.unix_timestamp);

        usdc_amount = pool
            .calculate_usdc_amount(mcc_amount)
            .ok_or(ReincarnationError::ArithmeticOverflow)?;

        require!(
            pool.can_execute_buyback(usdc_amount),
            ReincarnationError::DailyLimitExceeded
        );
    }

    require!(
        ctx.accounts.usdc_vault.amount >= usdc_amount,
        ReincarnationError::InsufficientPoolBalance
    );

    require!(
        ctx.accounts.user_mcc_account.amount >= mcc_amount,
        ReincarnationError::InsufficientMccBalance
    );

    do_transfer_checked(
        ctx.accounts.user_mcc_account.to_account_info(),
        ctx.accounts.mcc_vault.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.mcc_token_program.to_account_info(),
        mcc_amount,
        MCC_DECIMALS,
    )?;

    let pool_seeds = &[
        REINCARNATION_POOL_SEED,
        &[pool_bump],
    ];
    let signer_seeds = &[&pool_seeds[..]];

    do_transfer_checked_signed(
        ctx.accounts.usdc_vault.to_account_info(),
        ctx.accounts.user_usdc_account.to_account_info(),
        ctx.accounts.reincarnation_pool.to_account_info(),
        ctx.accounts.usdc_mint.to_account_info(),
        ctx.accounts.usdc_token_program.to_account_info(),
        usdc_amount,
        USDC_DECIMALS,
        signer_seeds,
    )?;

    let pool = &mut ctx.accounts.reincarnation_pool;
    pool.daily_used = pool.daily_used.saturating_add(usdc_amount);
    pool.total_mcc_bought = pool.total_mcc_bought.saturating_add(mcc_amount);
    pool.total_usd_paid = pool.total_usd_paid.saturating_add(usdc_amount);
    pool.total_buyback_count = pool.total_buyback_count.saturating_add(1);

    let buyback_price = pool.calculate_buyback_price();
    msg!("Buyback executed:");
    msg!("  User: {}", ctx.accounts.user.key());
    msg!("  MCC amount: {} (9 decimals)", mcc_amount);
    msg!("  USDC amount: {} (6 decimals)", usdc_amount);
    msg!("  Buyback price: {} USDC per MCC", buyback_price);
    msg!("  Total buybacks: {}", pool.total_buyback_count);

    emit!(BuybackExecuted {
        user: ctx.accounts.user.key(),
        mcc_amount,
        usdc_amount,
        buyback_price,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteBuyback<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = mcc_mint.key() == reincarnation_pool.mcc_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        constraint = (
            usdc_mint.key() == reincarnation_pool.usdc_mint
            || usdc_mint.key() == reincarnation_pool.usdt_mint
        ) @ ReincarnationError::InvalidUsdcMint
    )]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = user_mcc_account.owner == user.key(),
        constraint = user_mcc_account.mint == mcc_mint.key(),
    )]
    pub user_mcc_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_usdc_account.owner == user.key(),
        constraint = user_usdc_account.mint == usdc_mint.key(),
    )]
    pub user_usdc_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = (
            usdc_vault.key() == reincarnation_pool.usdc_vault
            || usdc_vault.key() == reincarnation_pool.usdt_vault
        ),
    )]
    pub usdc_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mcc_vault.key() == reincarnation_pool.mcc_vault @ ReincarnationError::InvalidMccMint,
    )]
    pub mcc_vault: InterfaceAccount<'info, TokenAccount>,

    pub mcc_token_program: Interface<'info, TokenInterface>,

    pub usdc_token_program: Program<'info, Token>,
}

#[event]
pub struct BuybackExecuted {
    pub user: Pubkey,
    pub mcc_amount: u64,
    pub usdc_amount: u64,
    pub buyback_price: u64,
    pub timestamp: i64,
}
