// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, Burn, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentConfig, FragmentVault, VaultStatus};

#[derive(Accounts)]
pub struct RedeemNftBurnFragments<'info> {
    #[account(mut)]
    pub redeemer: Signer<'info>,

    #[account(
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump,
        constraint = !config.is_paused @ FragmentError::OperationPaused
    )]
    pub config: Account<'info, FragmentConfig>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::Active @ FragmentError::VaultNotActive
    )]
    pub vault: Account<'info, FragmentVault>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [FRAGMENT_MINT_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub fragment_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = redeemer_fragment_account.owner == redeemer.key(),
        constraint = redeemer_fragment_account.mint == fragment_mint.key(),
        constraint = redeemer_fragment_account.amount == vault.total_fragments @ FragmentError::InsufficientFragments
    )]
    pub redeemer_fragment_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler_burn_fragments(ctx: Context<RedeemNftBurnFragments>) -> Result<()> {
    let total_fragments = ctx.accounts.vault.total_fragments;

    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.fragment_mint.to_account_info(),
            from: ctx.accounts.redeemer_fragment_account.to_account_info(),
            authority: ctx.accounts.redeemer.to_account_info(),
        },
    );
    token::burn(burn_ctx, total_fragments)?;

    let vault = &mut ctx.accounts.vault;
    vault.status = VaultStatus::Redeeming;

    msg!("Step 1: Fragments burned successfully!");
    msg!("Redeemer: {}", ctx.accounts.redeemer.key());
    msg!("Fragments burned: {}", total_fragments);

    Ok(())
}

#[derive(Accounts)]
pub struct RedeemNftFinalize<'info> {
    #[account(mut)]
    pub redeemer: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, FragmentConfig>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::Redeeming @ FragmentError::InvalidVaultStatus,
        constraint = vault.original_owner == redeemer.key() @ FragmentError::NotOriginalOwner
    )]
    pub vault: Account<'info, FragmentVault>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = vault,
        constraint = nft_escrow.amount == 1 @ FragmentError::NftNotInEscrow
    )]
    pub nft_escrow: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = redeemer,
        associated_token::mint = nft_mint,
        associated_token::authority = redeemer
    )]
    pub redeemer_nft_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_finalize(ctx: Context<RedeemNftFinalize>) -> Result<()> {
    let clock = Clock::get()?;

    let nft_mint_key = ctx.accounts.nft_mint.key();
    let vault_bump = ctx.accounts.vault.bump;
    let vault_seeds = &[
        FRAGMENT_VAULT_SEED,
        nft_mint_key.as_ref(),
        &[vault_bump],
    ];
    let signer_seeds = &[&vault_seeds[..]];

    let transfer_nft_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.nft_escrow.to_account_info(),
            to: ctx.accounts.redeemer_nft_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_nft_ctx, 1)?;

    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft_escrow.to_account_info(),
            destination: ctx.accounts.redeemer.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds,
    );
    token::close_account(close_ctx)?;

    let vault = &mut ctx.accounts.vault;
    vault.status = VaultStatus::Redeemed;
    vault.redeemed_at = Some(clock.unix_timestamp);
    vault.circulating_fragments = 0;

    let config = &mut ctx.accounts.config;
    config.total_fragmented_nfts = config.total_fragmented_nfts
        .checked_sub(1)
        .ok_or(FragmentError::MathOverflow)?;
    config.updated_at = clock.unix_timestamp;

    msg!("Step 2: NFT redeemed successfully!");
    msg!("NFT Mint: {}", ctx.accounts.nft_mint.key());
    msg!("Redeemer: {}", ctx.accounts.redeemer.key());

    Ok(())
}
