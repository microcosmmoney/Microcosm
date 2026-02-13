use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenInterface, TokenAccount, MintTo};

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, StationMcdVault};

#[derive(Accounts)]
#[instruction(station_id: u64)]
pub struct MintMiningMcd<'info> {
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
    ctx: Context<MintMiningMcd>,
    station_id: u64,
    mcd_amount: u64,
) -> Result<()> {
    require!(mcd_amount > 0, McdError::InvalidAmount);

    let config = &mut ctx.accounts.mcd_config;
    let vault = &mut ctx.accounts.station_vault;

    msg!("Mint Mining MCD");
    msg!("Station ID: {}", station_id);
    msg!("MCD amount: {} MCD", mcd_amount as f64 / 1_000_000.0);

    let config_seeds = &[
        MCD_CONFIG_SEED,
        &[config.bump],
    ];
    let signer = &[&config_seeds[..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mcd_mint.to_account_info(),
            to: ctx.accounts.station_token_account.to_account_info(),
            authority: config.to_account_info(),
        },
        signer,
    );
    token_interface::mint_to(mint_ctx, mcd_amount)?;

    config.total_minted = config.total_minted
        .checked_add(mcd_amount)
        .ok_or(McdError::MathOverflow)?;
    config.total_vault_minted = config.total_vault_minted
        .checked_add(mcd_amount)
        .ok_or(McdError::MathOverflow)?;

    vault.balance = vault.balance
        .checked_add(mcd_amount)
        .ok_or(McdError::MathOverflow)?;
    vault.total_received = vault.total_received
        .checked_add(mcd_amount)
        .ok_or(McdError::MathOverflow)?;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    msg!("MCD minted to Station vault: {} MCD", mcd_amount as f64 / 1_000_000.0);
    msg!("Station vault balance: {} MCD", vault.balance as f64 / 1_000_000.0);

    Ok(())
}
