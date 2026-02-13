use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

use crate::constants::*;
use crate::error::DhcError;
use crate::instructions::dao_create::{DaoFundraise, DaoContribution};

pub const CONTRIBUTION_FEE_BPS: u64 = 100;
pub const BPS_DIVISOR: u64 = 10000;

pub fn handler(
    ctx: Context<DaoContribute>,
    fundraise_id: u64,
    contributor_uid: u64,
    mcc_amount: u64,
) -> Result<()> {
    let fundraise = &ctx.accounts.dao_fundraise;
    let clock = Clock::get()?;

    require!(fundraise.status == 0, DhcError::Unauthorized);
    require!(clock.unix_timestamp < fundraise.deadline, DhcError::Unauthorized);

    require!(
        mcc_amount >= fundraise.min_contribution && mcc_amount <= fundraise.max_contribution,
        DhcError::InvalidAmount
    );

    let remaining = fundraise.target_amount.saturating_sub(fundraise.raised_amount);
    let actual_amount = mcc_amount.min(remaining);
    require!(actual_amount > 0, DhcError::InvalidAmount);

    let fee = actual_amount
        .checked_mul(CONTRIBUTION_FEE_BPS)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(BPS_DIVISOR)
        .ok_or(DhcError::MathOverflow)?;

    let net_contribution = actual_amount
        .checked_sub(fee)
        .ok_or(DhcError::MathOverflow)?;

    msg!("DAO Contribute");
    msg!("Fundraise ID: {}", fundraise_id);
    msg!("Contributor UID: {}", contributor_uid);
    msg!("Amount: {} MCC", mcc_amount as f64 / 1_000_000_000.0);
    msg!("Fee: {} MCC", fee as f64 / 1_000_000_000.0);
    msg!("Net: {} MCC", net_contribution as f64 / 1_000_000_000.0);

    let transfer_pool_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.contributor_mcc_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.contributor.to_account_info(),
        },
    );
    token::transfer(transfer_pool_ctx, net_contribution)?;

    if fee > 0 {
        let transfer_fee_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.contributor_mcc_account.to_account_info(),
                to: ctx.accounts.team_vault.to_account_info(),
                authority: ctx.accounts.contributor.to_account_info(),
            },
        );
        token::transfer(transfer_fee_ctx, fee)?;
    }

    let fundraise = &mut ctx.accounts.dao_fundraise;
    fundraise.raised_amount = fundraise.raised_amount
        .checked_add(net_contribution)
        .ok_or(DhcError::MathOverflow)?;
    fundraise.contributor_count = fundraise.contributor_count
        .checked_add(1)
        .ok_or(DhcError::MathOverflow)?;

    let threshold = fundraise.target_amount
        .checked_mul(99)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(100)
        .ok_or(DhcError::MathOverflow)?;
    if fundraise.raised_amount >= threshold {
        fundraise.status = 1;
        msg!("Fundraise target reached! (99% threshold)");
    }

    let contribution = &mut ctx.accounts.dao_contribution;
    contribution.fundraise_id = fundraise_id;
    contribution.contributor_uid = contributor_uid;
    contribution.contributor = ctx.accounts.contributor.key();
    contribution.amount = net_contribution;
    contribution.share_bps = 0;
    contribution.claimed = false;
    contribution.created_at = clock.unix_timestamp;
    contribution.bump = ctx.bumps.dao_contribution;

    msg!("Contribution recorded: {} MCC", net_contribution as f64 / 1_000_000_000.0);
    msg!("Total raised: {} / {} MCC",
        fundraise.raised_amount as f64 / 1_000_000_000.0,
        fundraise.target_amount as f64 / 1_000_000_000.0);

    emit!(DaoContributeEvent {
        fundraise_id,
        contributor: ctx.accounts.contributor.key(),
        contributor_uid,
        amount: net_contribution,
        fee,
        total_raised: fundraise.raised_amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(fundraise_id: u64, contributor_uid: u64)]
pub struct DaoContribute<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dao_fundraise", fundraise_id.to_le_bytes().as_ref()],
        bump = dao_fundraise.bump,
    )]
    pub dao_fundraise: Account<'info, DaoFundraise>,

    #[account(
        init,
        payer = contributor,
        space = DaoContribution::LEN,
        seeds = [b"dao_contribution", fundraise_id.to_le_bytes().as_ref(), contributor_uid.to_le_bytes().as_ref()],
        bump,
    )]
    pub dao_contribution: Account<'info, DaoContribution>,

    #[account(
        mut,
        token::mint = mcc_mint,
        token::authority = contributor,
    )]
    pub contributor_mcc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool_token_account.key() == dao_fundraise.pool_token_account @ DhcError::Unauthorized,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub team_vault: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct DaoContributeEvent {
    pub fundraise_id: u64,
    pub contributor: Pubkey,
    pub contributor_uid: u64,
    pub amount: u64,
    pub fee: u64,
    pub total_raised: u64,
    pub timestamp: i64,
}
