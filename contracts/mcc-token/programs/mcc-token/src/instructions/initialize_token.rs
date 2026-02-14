// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use crate::constants::*;
use crate::error::DhcError;

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = DECIMALS,
        mint::authority = founder_vault,
    )]
    pub mcc_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = mcc_mint,
        token::authority = founder_vault,
        seeds = [FOUNDER_VAULT_SEED],
        bump,
    )]
    pub founder_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeToken>, decimals: u8) -> Result<()> {
    require!(decimals == DECIMALS, DhcError::InvalidMiningRate);

    msg!("Initializing MCC Token...");
    msg!("Mint address: {}", ctx.accounts.mcc_mint.key());
    msg!("Founder vault: {}", ctx.accounts.founder_vault.key());
    msg!("Decimal precision: {}", DECIMALS);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mcc_mint.to_account_info(),
        to: ctx.accounts.founder_vault.to_account_info(),
        authority: ctx.accounts.founder_vault.to_account_info(),
    };

    let seeds = &[
        FOUNDER_VAULT_SEED,
        &[ctx.bumps.founder_vault],
    ];
    let signer = &[&seeds[..]];

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::mint_to(cpi_ctx, TOTAL_SUPPLY)?;

    msg!("Successfully minted 1 billion MCC to founder vault");
    msg!("Total supply: {} ({} decimals)", TOTAL_SUPPLY, DECIMALS);

    Ok(())
}
