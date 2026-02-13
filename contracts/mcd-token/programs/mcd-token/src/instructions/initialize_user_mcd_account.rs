use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, UserMcdAccount};

#[derive(Accounts)]
#[instruction(uid: u64)]
pub struct InitializeUserMcdAccount<'info> {
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
        init,
        payer = authority,
        space = UserMcdAccount::LEN,
        seeds = [USER_MCD_SEED, &uid.to_le_bytes()],
        bump,
    )]
    pub user_mcd_account: Account<'info, UserMcdAccount>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mcd_mint,
        associated_token::authority = user_mcd_account,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<InitializeUserMcdAccount>, uid: u64) -> Result<()> {
    let user_pda_key = ctx.accounts.user_mcd_account.key();
    let token_account_key = ctx.accounts.user_token_account.key();
    let bump = ctx.bumps.user_mcd_account;

    let user_account = &mut ctx.accounts.user_mcd_account;

    user_account.uid = uid;
    user_account.station_id = 0;
    user_account.balance = 0;
    user_account.total_received = 0;
    user_account.total_spent = 0;
    user_account.total_deposited = 0;
    user_account.total_withdrawn = 0;
    user_account.token_account = token_account_key;
    user_account.bump = bump;

    msg!("Initialize User MCD Account");
    msg!("UID: {}", uid);
    msg!("User PDA: {}", user_pda_key);
    msg!("Token Account: {}", token_account_key);

    Ok(())
}
