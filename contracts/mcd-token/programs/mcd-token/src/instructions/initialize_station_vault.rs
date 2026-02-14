// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, StationMcdVault};

#[derive(Accounts)]
#[instruction(station_id: u64)]
pub struct InitializeStationVault<'info> {
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
        space = StationMcdVault::LEN,
        seeds = [STATION_VAULT_SEED, &station_id.to_le_bytes()],
        bump,
    )]
    pub station_vault: Account<'info, StationMcdVault>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mcd_mint,
        associated_token::authority = station_vault,
        associated_token::token_program = token_program,
    )]
    pub station_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<InitializeStationVault>, station_id: u64) -> Result<()> {
    let vault_key = ctx.accounts.station_vault.key();
    let token_account_key = ctx.accounts.station_token_account.key();
    let bump = ctx.bumps.station_vault;

    let vault = &mut ctx.accounts.station_vault;

    vault.station_id = station_id;
    vault.balance = 0;
    vault.total_received = 0;
    vault.total_distributed = 0;
    vault.last_distribution_date = 0;
    vault.token_account = token_account_key;
    vault.bump = bump;
    vault._reserved = [0u8; 32];

    msg!("Initialize Station MCD Vault");
    msg!("Station ID: {}", station_id);
    msg!("Vault PDA: {}", vault_key);
    msg!("Token Account: {}", token_account_key);

    Ok(())
}
