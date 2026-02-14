// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenInterface, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, UserMcdAccount, StationMcdVault};

#[derive(Accounts)]
#[instruction(uid: u64, station_id: u64)]
pub struct McdMiningPayment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MCD_CONFIG_SEED],
        bump = mcd_config.bump,
        constraint = mcd_config.authority == authority.key() @ McdError::Unauthorized,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    #[account(
        mut,
        constraint = mcd_mint.key() == mcd_config.mcd_mint @ McdError::InvalidMint,
    )]
    pub mcd_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [USER_MCD_SEED, &uid.to_le_bytes()],
        bump = user_account.bump,
        constraint = user_account.uid == uid @ McdError::InvalidUserId,
    )]
    pub user_account: Account<'info, UserMcdAccount>,

    #[account(
        mut,
        constraint = user_token_account.key() == user_account.token_account @ McdError::UserAccountNotFound,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = recycle_pool_token_account.owner == mcd_config.recycle_pool @ McdError::InvalidAmount,
    )]
    pub recycle_pool_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [STATION_VAULT_SEED, &station_id.to_le_bytes()],
        bump = station_vault.bump,
        constraint = station_vault.station_id == station_id @ McdError::InvalidStationId,
    )]
    pub station_vault: Account<'info, StationMcdVault>,

    #[account(
        mut,
        constraint = station_token_account.key() == station_vault.token_account @ McdError::StationVaultNotFound,
    )]
    pub station_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<McdMiningPayment>,
    uid: u64,
    station_id: u64,
    mcd_amount: u64,
) -> Result<()> {
    require!(mcd_amount > 0, McdError::InvalidAmount);

    let user_account = &mut ctx.accounts.user_account;
    let config = &mut ctx.accounts.mcd_config;
    let vault = &mut ctx.accounts.station_vault;

    require!(
        user_account.balance >= mcd_amount,
        McdError::InsufficientBalance
    );

    msg!("MCD Mining Payment");
    msg!("UID: {}", uid);
    msg!("Station ID: {}", station_id);
    msg!("MCD amount: {} MCD", mcd_amount as f64 / 1_000_000.0);

    let recycle_amount = mcd_amount
        .checked_mul(MCD_MINING_RECYCLE_RATE)
        .ok_or(McdError::MathOverflow)?
        .checked_div(MCD_MINING_RECYCLE_DIVISOR)
        .ok_or(McdError::MathOverflow)?;

    let vault_amount = mcd_amount
        .checked_sub(recycle_amount)
        .ok_or(McdError::MathOverflow)?;

    msg!("Recycle to genesis pool (70%): {} MCD", recycle_amount as f64 / 1_000_000.0);
    msg!("Mint to vault (30%): {} MCD", vault_amount as f64 / 1_000_000.0);

    let user_seeds = &[
        USER_MCD_SEED,
        &uid.to_le_bytes(),
        &[user_account.bump],
    ];
    let user_signer = &[&user_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.recycle_pool_token_account.to_account_info(),
            authority: user_account.to_account_info(),
        },
        user_signer,
    );
    token_interface::transfer(transfer_ctx, recycle_amount)?;

    let transfer_vault_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.station_token_account.to_account_info(),
            authority: user_account.to_account_info(),
        },
        user_signer,
    );
    token_interface::transfer(transfer_vault_ctx, vault_amount)?;

    user_account.balance = user_account.balance
        .checked_sub(mcd_amount)
        .ok_or(McdError::MathOverflow)?;
    user_account.total_spent = user_account.total_spent
        .checked_add(mcd_amount)
        .ok_or(McdError::MathOverflow)?;

    vault.balance = vault.balance
        .checked_add(vault_amount)
        .ok_or(McdError::MathOverflow)?;
    vault.total_received = vault.total_received
        .checked_add(vault_amount)
        .ok_or(McdError::MathOverflow)?;

    config.total_recycled = config.total_recycled
        .checked_add(recycle_amount)
        .ok_or(McdError::MathOverflow)?;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    msg!("MCD mining payment completed");
    msg!("User new balance: {} MCD", user_account.balance as f64 / 1_000_000.0);
    msg!("Vault new balance: {} MCD", vault.balance as f64 / 1_000_000.0);

    Ok(())
}
