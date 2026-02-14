// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenInterface, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, McdWhitelist, UserMcdAccount};

#[derive(Accounts)]
#[instruction(from_uid: u64, to_project_id: u64)]
pub struct ConsumeMcd<'info> {
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
        seeds = [USER_MCD_SEED, &from_uid.to_le_bytes()],
        bump = from_user_account.bump,
        constraint = from_user_account.uid == from_uid @ McdError::InvalidUserId,
    )]
    pub from_user_account: Account<'info, UserMcdAccount>,

    #[account(
        mut,
        constraint = from_user_token_account.key() == from_user_account.token_account @ McdError::UserAccountNotFound,
    )]
    pub from_user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [PROJECT_MCD_SEED, &to_project_id.to_le_bytes()],
        bump,
    )]
    pub to_project_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub to_project_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [MCD_WHITELIST_SEED, &to_project_id.to_le_bytes()],
        bump = whitelist.bump,
        constraint = whitelist.wallet_address == to_project_token_account.owner @ McdError::NotInWhitelist,
        constraint = whitelist.status == McdWhitelist::STATUS_ACTIVE @ McdError::ProjectSuspended,
    )]
    pub whitelist: Account<'info, McdWhitelist>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<ConsumeMcd>,
    from_uid: u64,
    to_project_id: u64,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, McdError::InvalidAmount);

    let user_account = &mut ctx.accounts.from_user_account;
    let config = &mut ctx.accounts.mcd_config;

    require!(
        user_account.balance >= amount,
        McdError::InsufficientBalance
    );

    msg!("Consume MCD");
    msg!("From UID: {}", from_uid);
    msg!("To Project: {}", to_project_id);
    msg!("Amount: {} MCD", amount as f64 / 1_000_000.0);

    let user_seeds = &[
        USER_MCD_SEED,
        &from_uid.to_le_bytes(),
        &[user_account.bump],
    ];
    let signer = &[&user_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from_user_token_account.to_account_info(),
            to: ctx.accounts.to_project_token_account.to_account_info(),
            authority: user_account.to_account_info(),
        },
        signer,
    );
    token_interface::transfer(transfer_ctx, amount)?;

    user_account.balance = user_account.balance
        .checked_sub(amount)
        .ok_or(McdError::MathOverflow)?;
    user_account.total_spent = user_account.total_spent
        .checked_add(amount)
        .ok_or(McdError::MathOverflow)?;

    config.total_consumed = config.total_consumed
        .checked_add(amount)
        .ok_or(McdError::MathOverflow)?;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    msg!("MCD consumed: {} MCD", amount as f64 / 1_000_000.0);
    msg!("User new balance: {} MCD", user_account.balance as f64 / 1_000_000.0);

    Ok(())
}
