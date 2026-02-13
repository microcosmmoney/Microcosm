use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

use crate::constants::*;
use crate::error::DhcError;
use crate::state::MiningConfig;

pub const CYCLE_PREMIUM_BPS: u64 = 500;

pub const CYCLE_FEE_REGISTERED_BPS: u64 = 100;

pub const CYCLE_FEE_UNREGISTERED_BPS: u64 = 200;

pub const BPS_DIVISOR: u64 = 10000;

pub fn handler(
    ctx: Context<Cycle>,
    mcc_amount: u64,
    is_registered: bool,
    min_usdc_out: u64,
) -> Result<()> {
    require!(!ctx.accounts.mining_config.is_halted, DhcError::SystemHalted);

    require!(mcc_amount > 0, DhcError::InvalidAmount);

    let mining_config = &ctx.accounts.mining_config;
    let mining_price = mining_config.current_price;

    let cycle_price = mining_price
        .checked_mul(BPS_DIVISOR + CYCLE_PREMIUM_BPS)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(BPS_DIVISOR)
        .ok_or(DhcError::MathOverflow)?;

    let gross_usdc = (mcc_amount as u128)
        .checked_mul(cycle_price as u128)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(1_000_000_000)
        .ok_or(DhcError::MathOverflow)? as u64;

    let fee_bps = if is_registered {
        CYCLE_FEE_REGISTERED_BPS
    } else {
        CYCLE_FEE_UNREGISTERED_BPS
    };

    let fee_usdc = gross_usdc
        .checked_mul(fee_bps)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(BPS_DIVISOR)
        .ok_or(DhcError::MathOverflow)?;

    let net_usdc = gross_usdc
        .checked_sub(fee_usdc)
        .ok_or(DhcError::MathOverflow)?;

    require!(net_usdc >= min_usdc_out, DhcError::SlippageExceeded);

    require!(
        ctx.accounts.usdc_pool.amount >= net_usdc,
        DhcError::InsufficientLiquidity
    );

    msg!("Cycle: {} MCC -> {} USDC (fee: {} USDC)",
        mcc_amount as f64 / 1_000_000_000.0,
        net_usdc as f64 / 1_000_000.0,
        fee_usdc as f64 / 1_000_000.0
    );

    let transfer_mcc_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_mcc_account.to_account_info(),
            to: ctx.accounts.mcc_recycle_pool.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(transfer_mcc_ctx, mcc_amount)?;

    let bump = ctx.bumps.cycle_authority;
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"cycle_authority",
        &[bump],
    ]];

    let transfer_usdc_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.usdc_pool.to_account_info(),
            to: ctx.accounts.user_usdc_account.to_account_info(),
            authority: ctx.accounts.cycle_authority.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_usdc_ctx, net_usdc)?;

    let mining_config = &mut ctx.accounts.mining_config;
    mining_config.total_cycled = mining_config.total_cycled
        .checked_add(mcc_amount)
        .ok_or(DhcError::MathOverflow)?;

    msg!("Cycle successful: user received {} USDC", net_usdc as f64 / 1_000_000.0);

    emit!(CycleEvent {
        user: ctx.accounts.user.key(),
        mcc_amount,
        gross_usdc,
        fee_usdc,
        net_usdc,
        cycle_price,
        is_registered,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Cycle<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [MINING_CONFIG_SEED],
        bump = mining_config.bump,
    )]
    pub mining_config: Account<'info, MiningConfig>,

    #[account(
        seeds = [b"cycle_authority"],
        bump,
    )]
    pub cycle_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = mcc_mint,
        token::authority = user,
    )]
    pub user_mcc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = user,
    )]
    pub user_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mcc_mint,
    )]
    pub mcc_recycle_pool: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = usdc_mint,
        token::authority = cycle_authority,
    )]
    pub usdc_pool: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub usdc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[event]
pub struct CycleEvent {
    pub user: Pubkey,
    pub mcc_amount: u64,
    pub gross_usdc: u64,
    pub fee_usdc: u64,
    pub net_usdc: u64,
    pub cycle_price: u64,
    pub is_registered: bool,
    pub timestamp: i64,
}
