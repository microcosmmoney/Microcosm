// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentConfig, FragmentVault, VaultStatus};

#[derive(Accounts)]
#[instruction(fragment_count: u64, fragment_name: String, fragment_symbol: String)]
pub struct FragmentizeNftInitVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump,
        constraint = !config.is_paused @ FragmentError::OperationPaused
    )]
    pub config: Account<'info, FragmentConfig>,

    #[account(
        init,
        payer = owner,
        space = FragmentVault::LEN,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub fragment_vault: Account<'info, FragmentVault>,

    pub nft_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn handler_init_vault(
    ctx: Context<FragmentizeNftInitVault>,
    fragment_count: u64,
    fragment_name: String,
    fragment_symbol: String,
) -> Result<()> {
    let config = &ctx.accounts.config;

    require!(
        config.validate_fragment_count(fragment_count),
        FragmentError::FragmentCountBelowMinimum
    );
    require!(
        fragment_name.len() <= MAX_FRAGMENT_NAME_LEN,
        FragmentError::FragmentNameTooLong
    );
    require!(
        fragment_symbol.len() <= MAX_FRAGMENT_SYMBOL_LEN,
        FragmentError::FragmentSymbolTooLong
    );

    let clock = Clock::get()?;

    let vault = &mut ctx.accounts.fragment_vault;
    vault.original_owner = ctx.accounts.owner.key();
    vault.nft_mint = ctx.accounts.nft_mint.key();
    vault.fragment_mint = Pubkey::default();
    vault.fragment_name = fragment_name.clone();
    vault.fragment_symbol = fragment_symbol.clone();
    vault.total_fragments = fragment_count;
    vault.circulating_fragments = 0;
    vault.status = VaultStatus::Initializing;
    vault.created_at = clock.unix_timestamp;
    vault.redeemed_at = None;
    vault.bump = ctx.bumps.fragment_vault;

    msg!("Fragment Vault initialized (Step 1/3)");
    msg!("NFT Mint: {}", ctx.accounts.nft_mint.key());
    msg!("Fragment count: {}", fragment_count);

    Ok(())
}

#[derive(Accounts)]
pub struct FragmentizeNftInitToken<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump = fragment_vault.bump,
        constraint = fragment_vault.original_owner == owner.key() @ FragmentError::NotOriginalOwner,
        constraint = fragment_vault.status == VaultStatus::Initializing @ FragmentError::InvalidVaultStatus
    )]
    pub fragment_vault: Box<Account<'info, FragmentVault>>,

    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = owner_nft_account.owner == owner.key(),
        constraint = owner_nft_account.mint == nft_mint.key(),
        constraint = owner_nft_account.amount == 1 @ FragmentError::NftNotOwned
    )]
    pub owner_nft_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = nft_mint,
        associated_token::authority = fragment_vault
    )]
    pub nft_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = owner,
        mint::decimals = FRAGMENT_DECIMALS,
        mint::authority = fragment_vault,
        seeds = [FRAGMENT_MINT_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub fragment_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_init_token(ctx: Context<FragmentizeNftInitToken>) -> Result<()> {
    let vault = &mut ctx.accounts.fragment_vault;
    vault.fragment_mint = ctx.accounts.fragment_mint.key();

    transfer_nft_to_escrow(&ctx)?;

    msg!("Fragment Mint and NFT Escrow initialized (Step 2/3)");
    msg!("Fragment Mint: {}", ctx.accounts.fragment_mint.key());
    msg!("NFT transferred to escrow");

    Ok(())
}

#[inline(never)]
fn transfer_nft_to_escrow(ctx: &Context<FragmentizeNftInitToken>) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.owner_nft_account.to_account_info(),
            to: ctx.accounts.nft_escrow.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, 1)?;
    Ok(())
}

#[derive(Accounts)]
pub struct FragmentizeNftFinalize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, FragmentConfig>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump = fragment_vault.bump,
        constraint = fragment_vault.original_owner == owner.key() @ FragmentError::NotOriginalOwner,
        constraint = fragment_vault.status == VaultStatus::Initializing @ FragmentError::InvalidVaultStatus,
        constraint = fragment_vault.fragment_mint != Pubkey::default() @ FragmentError::FragmentMintNotSet
    )]
    pub fragment_vault: Account<'info, FragmentVault>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [FRAGMENT_MINT_SEED, nft_mint.key().as_ref()],
        bump,
        constraint = fragment_mint.key() == fragment_vault.fragment_mint @ FragmentError::FragmentMintMismatch
    )]
    pub fragment_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = fragment_mint,
        associated_token::authority = owner
    )]
    pub owner_fragment_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_finalize(ctx: Context<FragmentizeNftFinalize>) -> Result<()> {
    let clock = Clock::get()?;
    let vault = &ctx.accounts.fragment_vault;

    mint_fragments_to_owner(&ctx, vault.total_fragments, vault.bump)?;

    let vault = &mut ctx.accounts.fragment_vault;
    vault.status = VaultStatus::Active;
    vault.circulating_fragments = vault.total_fragments;

    let config = &mut ctx.accounts.config;
    config.total_fragmented_nfts = config.total_fragmented_nfts
        .checked_add(1)
        .ok_or(FragmentError::MathOverflow)?;
    config.updated_at = clock.unix_timestamp;

    msg!("NFT fragmentation complete! (Step 3/3)");
    msg!("NFT Mint: {}", ctx.accounts.nft_mint.key());
    msg!("Fragment Mint: {}", ctx.accounts.fragment_mint.key());
    msg!("Total fragments minted: {}", vault.total_fragments);

    Ok(())
}

#[inline(never)]
fn mint_fragments_to_owner(
    ctx: &Context<FragmentizeNftFinalize>,
    amount: u64,
    vault_bump: u8,
) -> Result<()> {
    let nft_mint_key = ctx.accounts.nft_mint.key();
    let vault_seeds = &[
        FRAGMENT_VAULT_SEED,
        nft_mint_key.as_ref(),
        &[vault_bump],
    ];
    let signer_seeds = &[&vault_seeds[..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.fragment_mint.to_account_info(),
            to: ctx.accounts.owner_fragment_account.to_account_info(),
            authority: ctx.accounts.fragment_vault.to_account_info(),
        },
        signer_seeds,
    );
    token::mint_to(mint_ctx, amount)?;
    Ok(())
}
