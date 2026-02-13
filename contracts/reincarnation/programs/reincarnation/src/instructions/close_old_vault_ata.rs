use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use anchor_spl::token_interface::{TokenAccount, TokenInterface, CloseAccount};

use crate::constants::REINCARNATION_POOL_SEED;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

#[derive(Accounts)]
pub struct CloseOldVaultAta<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [REINCARNATION_POOL_SEED],
        bump = pool.bump,
        constraint = pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub pool: Account<'info, ReincarnationPool>,

    #[account(
        mut,
        constraint = old_token_account.owner == pool.key() @ ReincarnationError::InvalidVault,
        constraint = old_token_account.amount == 0 @ ReincarnationError::VaultNotEmpty,
    )]
    pub old_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<CloseOldVaultAta>) -> Result<()> {
    let old_ata = ctx.accounts.old_token_account.key();
    let authority = ctx.accounts.authority.key();
    let pool_bump = ctx.accounts.pool.bump;

    msg!("Closing old Vault ATA");
    msg!("Old ATA: {}", old_ata);
    msg!("Rent receiver: {}", authority);

    let signer_seeds: &[&[&[u8]]] = &[&[
        REINCARNATION_POOL_SEED,
        &[pool_bump],
    ]];

    let cpi_accounts = CloseAccount {
        account: ctx.accounts.old_token_account.to_account_info(),
        destination: ctx.accounts.authority.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token_2022::close_account(cpi_ctx)?;

    msg!("Old ATA closed successfully, rent recovered");

    Ok(())
}
