// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

use crate::error::DhcError;
use crate::instructions::dao_create::{DaoFundraise, DaoContribution};

pub const DAO_STATUS_ACTIVE: u8 = 0;
pub const DAO_STATUS_SUCCESS: u8 = 1;
pub const DAO_STATUS_FAILED: u8 = 2;
pub const DAO_STATUS_EXECUTED: u8 = 3;

pub fn handler_settle(ctx: Context<DaoSettle>, fundraise_id: u64) -> Result<()> {
    let fundraise = &ctx.accounts.dao_fundraise;
    let clock = Clock::get()?;

    msg!("DAO Settle");
    msg!("Fundraise ID: {}", fundraise_id);
    msg!("Status: {}", fundraise.status);
    msg!("Raised: {} / {} MCC",
        fundraise.raised_amount as f64 / 1_000_000_000.0,
        fundraise.target_amount as f64 / 1_000_000_000.0);

    require!(
        fundraise.initiator == ctx.accounts.initiator.key(),
        DhcError::Unauthorized
    );

    let threshold = fundraise.target_amount
        .checked_mul(99)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(100)
        .ok_or(DhcError::MathOverflow)?;
    let is_success = fundraise.status == DAO_STATUS_SUCCESS ||
        fundraise.raised_amount >= threshold;
    require!(is_success, DhcError::Unauthorized);

    require!(fundraise.status != DAO_STATUS_EXECUTED, DhcError::Unauthorized);

    let pool_balance = ctx.accounts.pool_token_account.amount;
    require!(pool_balance > 0, DhcError::InvalidAmount);

    msg!("Pool balance: {} MCC", pool_balance as f64 / 1_000_000_000.0);

    let fundraise_id_bytes = fundraise_id.to_le_bytes();
    let seeds = &[
        b"dao_fundraise".as_ref(),
        fundraise_id_bytes.as_ref(),
        &[fundraise.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.initiator_mcc_account.to_account_info(),
            authority: ctx.accounts.dao_fundraise.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, pool_balance)?;

    let fundraise = &mut ctx.accounts.dao_fundraise;
    fundraise.status = DAO_STATUS_EXECUTED;

    msg!("Settlement complete! {} MCC transferred to initiator",
        pool_balance as f64 / 1_000_000_000.0);

    emit!(DaoSettleEvent {
        fundraise_id,
        initiator: ctx.accounts.initiator.key(),
        amount: pool_balance,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(fundraise_id: u64)]
pub struct DaoSettle<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dao_fundraise", fundraise_id.to_le_bytes().as_ref()],
        bump = dao_fundraise.bump,
    )]
    pub dao_fundraise: Account<'info, DaoFundraise>,

    #[account(
        mut,
        constraint = pool_token_account.key() == dao_fundraise.pool_token_account @ DhcError::Unauthorized,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mcc_mint,
        token::authority = initiator,
    )]
    pub initiator_mcc_account: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler_refund(
    ctx: Context<DaoRefund>,
    fundraise_id: u64,
    contributor_uid: u64,
) -> Result<()> {
    let fundraise = &ctx.accounts.dao_fundraise;
    let contribution = &ctx.accounts.dao_contribution;
    let clock = Clock::get()?;

    msg!("DAO Refund");
    msg!("Fundraise ID: {}", fundraise_id);
    msg!("Contributor UID: {}", contributor_uid);
    msg!("Contribution: {} MCC", contribution.amount as f64 / 1_000_000_000.0);

    require!(
        contribution.contributor == ctx.accounts.contributor.key(),
        DhcError::Unauthorized
    );

    require!(!contribution.claimed, DhcError::Unauthorized);

    let is_failed = fundraise.status == DAO_STATUS_FAILED ||
        (clock.unix_timestamp >= fundraise.deadline &&
         fundraise.raised_amount < fundraise.target_amount);
    require!(is_failed, DhcError::Unauthorized);

    let refund_amount = contribution.amount;
    require!(refund_amount > 0, DhcError::InvalidAmount);

    require!(
        ctx.accounts.pool_token_account.amount >= refund_amount,
        DhcError::InvalidAmount
    );

    msg!("Refund amount: {} MCC", refund_amount as f64 / 1_000_000_000.0);

    let fundraise_id_bytes = fundraise_id.to_le_bytes();
    let seeds = &[
        b"dao_fundraise".as_ref(),
        fundraise_id_bytes.as_ref(),
        &[fundraise.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.contributor_mcc_account.to_account_info(),
            authority: ctx.accounts.dao_fundraise.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, refund_amount)?;

    let contribution = &mut ctx.accounts.dao_contribution;
    contribution.claimed = true;

    let fundraise = &mut ctx.accounts.dao_fundraise;
    if fundraise.status == DAO_STATUS_ACTIVE {
        fundraise.status = DAO_STATUS_FAILED;
    }

    msg!("Refund complete! {} MCC returned to contributor",
        refund_amount as f64 / 1_000_000_000.0);

    emit!(DaoRefundEvent {
        fundraise_id,
        contributor: ctx.accounts.contributor.key(),
        contributor_uid,
        amount: refund_amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(fundraise_id: u64, contributor_uid: u64)]
pub struct DaoRefund<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dao_fundraise", fundraise_id.to_le_bytes().as_ref()],
        bump = dao_fundraise.bump,
    )]
    pub dao_fundraise: Account<'info, DaoFundraise>,

    #[account(
        mut,
        seeds = [b"dao_contribution", fundraise_id.to_le_bytes().as_ref(), contributor_uid.to_le_bytes().as_ref()],
        bump = dao_contribution.bump,
    )]
    pub dao_contribution: Account<'info, DaoContribution>,

    #[account(
        mut,
        constraint = pool_token_account.key() == dao_fundraise.pool_token_account @ DhcError::Unauthorized,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mcc_mint,
        token::authority = contributor,
    )]
    pub contributor_mcc_account: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[event]
pub struct DaoSettleEvent {
    pub fundraise_id: u64,
    pub initiator: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct DaoRefundEvent {
    pub fundraise_id: u64,
    pub contributor: Pubkey,
    pub contributor_uid: u64,
    pub amount: u64,
    pub timestamp: i64,
}
