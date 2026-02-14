// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenInterface, TokenAccount, MintTo};

use crate::constants::*;
use crate::error::McdError;
use crate::state::McdConfig;

#[derive(Accounts)]
pub struct MintGenesisSupply<'info> {
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
        constraint = genesis_pool_ata.mint == mcd_mint.key() @ McdError::InvalidMint,
    )]
    pub genesis_pool_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<MintGenesisSupply>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, McdError::InvalidAmount);

    let config = &mut ctx.accounts.mcd_config;

    msg!("Mint Genesis Supply");
    msg!("Amount: {} MCD", amount as f64 / 1_000_000.0);
    msg!("Genesis Pool ATA: {}", ctx.accounts.genesis_pool_ata.key());

    let config_seeds = &[
        MCD_CONFIG_SEED,
        &[config.bump],
    ];
    let signer = &[&config_seeds[..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mcd_mint.to_account_info(),
            to: ctx.accounts.genesis_pool_ata.to_account_info(),
            authority: config.to_account_info(),
        },
        signer,
    );
    token_interface::mint_to(mint_ctx, amount)?;

    config.total_minted = config.total_minted
        .checked_add(amount)
        .ok_or(McdError::MathOverflow)?;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    msg!("Genesis supply minted successfully");
    msg!("Total MCD minted: {} MCD", config.total_minted as f64 / 1_000_000.0);

    Ok(())
}
