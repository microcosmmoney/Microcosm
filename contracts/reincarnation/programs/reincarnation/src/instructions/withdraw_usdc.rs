// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(
    ctx: Context<WithdrawUsdc>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, ReincarnationError::ZeroAmount);

    require!(
        ctx.accounts.usdc_vault.amount >= amount,
        ReincarnationError::InsufficientPoolBalance
    );

    let pool = &ctx.accounts.reincarnation_pool;

    let pool_seeds = &[
        REINCARNATION_POOL_SEED,
        &[pool.bump],
    ];
    let signer_seeds = &[&pool_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.usdc_vault.to_account_info(),
        to: ctx.accounts.authority_usdc_account.to_account_info(),
        authority: ctx.accounts.reincarnation_pool.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    let clock = Clock::get()?;
    msg!("USDC withdrawn:");
    msg!("  Amount: {} (6 decimals)", amount);
    msg!("  To: {}", ctx.accounts.authority.key());
    msg!("  Remaining: {}", ctx.accounts.usdc_vault.amount - amount);

    emit!(UsdcWithdrawn {
        amount,
        to: ctx.accounts.authority.key(),
        remaining: ctx.accounts.usdc_vault.amount - amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawUsdc<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = usdc_mint.key() == reincarnation_pool.usdc_mint @ ReincarnationError::InvalidUsdcMint
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = usdc_vault.key() == reincarnation_pool.usdc_vault,
    )]
    pub usdc_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = authority_usdc_account.owner == authority.key(),
        constraint = authority_usdc_account.mint == usdc_mint.key(),
    )]
    pub authority_usdc_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[event]
pub struct UsdcWithdrawn {
    pub amount: u64,
    pub to: Pubkey,
    pub remaining: u64,
    pub timestamp: i64,
}
