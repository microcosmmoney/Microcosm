// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

use crate::error::DhcError;

pub const DAO_CREATE_FEE: u64 = 1_000_000_000_000;

#[account]
pub struct DaoFundraise {
    pub id: u64,

    pub initiator_uid: u64,

    pub initiator: Pubkey,

    pub station_id: u64,

    pub target_amount: u64,

    pub min_contribution: u64,

    pub max_contribution: u64,

    pub raised_amount: u64,

    pub contributor_count: u64,

    pub deadline: i64,

    pub status: u8,

    pub pool_token_account: Pubkey,

    pub created_at: i64,

    pub pool_initialized: bool,

    pub bump: u8,
}

impl DaoFundraise {
    pub const LEN: usize = 8 +
        8 +
        8 +
        32 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        1 +
        32 +
        8 +
        1 +
        1;
}

#[account]
pub struct DaoContribution {
    pub fundraise_id: u64,

    pub contributor_uid: u64,

    pub contributor: Pubkey,

    pub amount: u64,

    pub share_bps: u64,

    pub claimed: bool,

    pub created_at: i64,

    pub bump: u8,
}

impl DaoContribution {
    pub const LEN: usize = 8 +
        8 +
        8 +
        32 +
        8 +
        8 +
        1 +
        8 +
        1;
}

pub fn handler_init(
    ctx: Context<DaoCreateInit>,
    fundraise_id: u64,
    station_id: u64,
    target_amount: u64,
    min_contribution: u64,
    max_contribution: u64,
    deadline_seconds: u64,
) -> Result<()> {
    require!(target_amount > 0, DhcError::InvalidAmount);
    require!(min_contribution > 0 && min_contribution <= max_contribution, DhcError::InvalidAmount);
    require!(max_contribution <= target_amount, DhcError::InvalidAmount);
    require!(deadline_seconds >= 60 && deadline_seconds <= 2592000, DhcError::InvalidAmount);

    let clock = Clock::get()?;
    let deadline = clock.unix_timestamp + (deadline_seconds as i64);

    msg!("DAO Create Step 1: Initialize DaoFundraise");
    msg!("ID: {}", fundraise_id);
    msg!("Station ID: {}", station_id);
    msg!("Target: {} MCC", target_amount as f64 / 1_000_000_000.0);
    msg!("Deadline: {} seconds", deadline_seconds);

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.initiator_mcc_account.to_account_info(),
            to: ctx.accounts.team_vault.to_account_info(),
            authority: ctx.accounts.initiator.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, DAO_CREATE_FEE)?;
    msg!("Creation fee transferred: 1000 MCC");

    let fundraise = &mut ctx.accounts.dao_fundraise;
    fundraise.id = fundraise_id;
    fundraise.initiator_uid = ctx.accounts.initiator.key().to_bytes()[0..8]
        .try_into()
        .map_or(0, |b| u64::from_le_bytes(b));
    fundraise.initiator = ctx.accounts.initiator.key();
    fundraise.station_id = station_id;
    fundraise.target_amount = target_amount;
    fundraise.min_contribution = min_contribution;
    fundraise.max_contribution = max_contribution;
    fundraise.raised_amount = 0;
    fundraise.contributor_count = 0;
    fundraise.deadline = deadline;
    fundraise.status = 0;
    fundraise.pool_token_account = Pubkey::default();
    fundraise.created_at = clock.unix_timestamp;
    fundraise.pool_initialized = false;
    fundraise.bump = ctx.bumps.dao_fundraise;

    msg!("DaoFundraise created, waiting for pool initialization");

    Ok(())
}

#[derive(Accounts)]
#[instruction(fundraise_id: u64, station_id: u64)]
pub struct DaoCreateInit<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    #[account(
        init,
        payer = initiator,
        space = DaoFundraise::LEN,
        seeds = [b"dao_fundraise", fundraise_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub dao_fundraise: Account<'info, DaoFundraise>,

    #[account(
        mut,
        token::mint = mcc_mint,
        token::authority = initiator,
    )]
    pub initiator_mcc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub team_vault: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler_pool(ctx: Context<DaoCreatePool>, fundraise_id: u64) -> Result<()> {
    msg!("DAO Create Step 2: Initialize Pool Token Account");
    msg!("Fundraise ID: {}", fundraise_id);

    let fundraise = &ctx.accounts.dao_fundraise;
    require!(
        fundraise.initiator == ctx.accounts.initiator.key(),
        DhcError::Unauthorized
    );
    require!(!fundraise.pool_initialized, DhcError::PoolAlreadyInitialized);

    let fundraise = &mut ctx.accounts.dao_fundraise;
    fundraise.pool_token_account = ctx.accounts.pool_token_account.key();
    fundraise.pool_initialized = true;

    msg!("Pool Token Account initialized: {}", fundraise.pool_token_account);

    let clock = Clock::get()?;
    emit!(DaoCreateEvent {
        fundraise_id,
        initiator: ctx.accounts.initiator.key(),
        station_id: fundraise.station_id,
        target_amount: fundraise.target_amount,
        deadline: fundraise.deadline,
        timestamp: clock.unix_timestamp,
    });

    msg!("DAO Fundraise fully created: {}", fundraise_id);

    Ok(())
}

#[derive(Accounts)]
#[instruction(fundraise_id: u64)]
pub struct DaoCreatePool<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dao_fundraise", fundraise_id.to_le_bytes().as_ref()],
        bump = dao_fundraise.bump,
    )]
    pub dao_fundraise: Account<'info, DaoFundraise>,

    #[account(
        init,
        payer = initiator,
        token::mint = mcc_mint,
        token::authority = dao_fundraise,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    _ctx: Context<DaoCreate>,
    _fundraise_id: u64,
    _station_id: u64,
    _target_amount: u64,
    _min_contribution: u64,
    _max_contribution: u64,
    _deadline_days: u64,
) -> Result<()> {
    msg!("ERROR: This instruction is deprecated due to stack overflow");
    msg!("Please use dao_create_init + dao_create_pool instead");
    Err(DhcError::StackOverflow.into())
}

#[derive(Accounts)]
#[instruction(fundraise_id: u64, station_id: u64)]
pub struct DaoCreate<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    #[account(
        init,
        payer = initiator,
        space = DaoFundraise::LEN,
        seeds = [b"dao_fundraise", fundraise_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub dao_fundraise: Account<'info, DaoFundraise>,

    #[account(
        mut,
        token::mint = mcc_mint,
        token::authority = initiator,
    )]
    pub initiator_mcc_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = initiator,
        token::mint = mcc_mint,
        token::authority = dao_fundraise,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub team_vault: Account<'info, TokenAccount>,

    pub mcc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct DaoCreateEvent {
    pub fundraise_id: u64,
    pub initiator: Pubkey,
    pub station_id: u64,
    pub target_amount: u64,
    pub deadline: i64,
    pub timestamp: i64,
}
