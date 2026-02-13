use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    Mint as InterfaceMint,
    TokenAccount as InterfaceTokenAccount,
    TokenInterface
};

use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(ctx: Context<InitializeVaults>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;

    pool.usdc_vault = ctx.accounts.usdc_vault.key();

    msg!("USDC Vault initialized: {}", pool.usdc_vault);
    msg!("Note: MCC/MCD vaults need separate initialization with Token-2022");

    Ok(())
}

pub fn handler_mcc_vault(ctx: Context<InitializeMccVault>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;

    pool.mcc_vault = ctx.accounts.mcc_vault.key();

    msg!("MCC Vault initialized: {}", pool.mcc_vault);

    Ok(())
}

pub fn handler_mcd_vault(ctx: Context<InitializeMcdVault>) -> Result<()> {
    let pool = &mut ctx.accounts.reincarnation_pool;

    pool.mcd_vault = ctx.accounts.mcd_vault.key();

    msg!("MCD Vault initialized: {}", pool.mcd_vault);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVaults<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = usdc_mint.key() == reincarnation_pool.usdc_mint @ ReincarnationError::InvalidUsdcMint
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = usdc_mint,
        associated_token::authority = reincarnation_pool,
    )]
    pub usdc_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct InitializeMccVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = mcc_mint.key() == reincarnation_pool.mcc_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcc_mint: InterfaceAccount<'info, InterfaceMint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mcc_mint,
        associated_token::authority = reincarnation_pool,
        associated_token::token_program = token_program,
    )]
    pub mcc_vault: InterfaceAccount<'info, InterfaceTokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct InitializeMcdVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
        constraint = reincarnation_pool.authority == authority.key() @ ReincarnationError::Unauthorized,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = mcd_mint.key() == reincarnation_pool.mcd_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcd_mint: InterfaceAccount<'info, InterfaceMint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mcd_mint,
        associated_token::authority = reincarnation_pool,
        associated_token::token_program = token_program,
    )]
    pub mcd_vault: InterfaceAccount<'info, InterfaceTokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
