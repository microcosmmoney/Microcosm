// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, CloseAccount, Mint, TokenInterface, TokenAccount};

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, UserMcdAccount};

#[derive(Accounts)]
#[instruction(uid: u64)]
pub struct CloseUserMcdAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MCD_CONFIG_SEED],
        bump = mcd_config.bump,
        constraint = mcd_config.authority == authority.key() @ McdError::Unauthorized,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    #[account(
        constraint = mcd_mint.key() == mcd_config.mcd_mint @ McdError::InvalidMint,
    )]
    pub mcd_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [USER_MCD_SEED, &uid.to_le_bytes()],
        bump = user_mcd_account.bump,
        constraint = user_mcd_account.uid == uid @ McdError::InvalidUserId,
        close = rent_receiver,
    )]
    pub user_mcd_account: Account<'info, UserMcdAccount>,

    #[account(
        mut,
        constraint = user_token_account.key() == user_mcd_account.token_account @ McdError::UserAccountNotFound,
        constraint = user_token_account.amount == 0 @ McdError::AccountNotEmpty,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub rent_receiver: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseUserMcdAccount>, uid: u64) -> Result<()> {
    let user_pda_key = ctx.accounts.user_mcd_account.key();
    let token_account_key = ctx.accounts.user_token_account.key();
    let rent_receiver_key = ctx.accounts.rent_receiver.key();

    let token_account_lamports = ctx.accounts.user_token_account.to_account_info().lamports();

    let bump = ctx.accounts.user_mcd_account.bump;
    let uid_bytes = uid.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[&[USER_MCD_SEED, &uid_bytes, &[bump]]];

    let cpi_accounts = CloseAccount {
        account: ctx.accounts.user_token_account.to_account_info(),
        destination: ctx.accounts.rent_receiver.to_account_info(),
        authority: ctx.accounts.user_mcd_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    token_interface::close_account(cpi_ctx)?;

    msg!("=== Close User MCD Account ===");
    msg!("UID: {}", uid);
    msg!("User PDA closed: {}", user_pda_key);
    msg!("Token Account closed: {}", token_account_key);
    msg!("Token Account rent recovered: {} lamports", token_account_lamports);
    msg!("Rent receiver: {}", rent_receiver_key);

    Ok(())
}
