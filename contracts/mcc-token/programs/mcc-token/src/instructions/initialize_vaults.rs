// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeVaults<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub mcc_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = mcc_mint,
        token::authority = team_vault,
        seeds = [TEAM_VAULT_SEED],
        bump,
    )]
    pub team_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = mcc_mint,
        token::authority = treasury_vault,
        seeds = [TREASURY_VAULT_SEED],
        bump,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeVaults>) -> Result<()> {
    msg!("Initializing Team and Treasury Vaults...");
    msg!("MCC Mint: {}", ctx.accounts.mcc_mint.key());
    msg!("Team Vault: {}", ctx.accounts.team_vault.key());
    msg!("Treasury Vault: {}", ctx.accounts.treasury_vault.key());

    msg!("Vaults initialized successfully");
    Ok(())
}
