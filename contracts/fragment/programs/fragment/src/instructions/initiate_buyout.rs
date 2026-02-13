use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentConfig, FragmentVault, Buyout, BuyoutStatus, VaultStatus};

#[derive(Accounts)]
pub struct InitiateBuyout<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump,
        constraint = !config.is_paused @ FragmentError::OperationPaused
    )]
    pub config: Box<Account<'info, FragmentConfig>>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::Active @ FragmentError::VaultNotActive
    )]
    pub vault: Box<Account<'info, FragmentVault>>,

    #[account(
        init,
        payer = initiator,
        space = Buyout::LEN,
        seeds = [BUYOUT_SEED, vault.key().as_ref()],
        bump
    )]
    pub buyout: Box<Account<'info, Buyout>>,

    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
        seeds = [FRAGMENT_MINT_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub fragment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = initiator_fragment_account.owner == initiator.key(),
        constraint = initiator_fragment_account.mint == fragment_mint.key()
    )]
    pub initiator_fragment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = initiator,
        associated_token::mint = fragment_mint,
        associated_token::authority = buyout
    )]
    pub buyout_fragment_escrow: Box<Account<'info, TokenAccount>>,

    pub payment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = initiator_payment_account.owner == initiator.key(),
        constraint = initiator_payment_account.mint == payment_mint.key()
    )]
    pub initiator_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = initiator,
        associated_token::mint = payment_mint,
        associated_token::authority = buyout
    )]
    pub buyout_payment_escrow: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitiateBuyout>,
    price_per_fragment: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let buyout_duration = ctx.accounts.config.buyout_duration;
    let vault_total_fragments = ctx.accounts.vault.total_fragments;

    require!(price_per_fragment > 0, FragmentError::InvalidBuyoutPrice);

    let initiator_fragments = ctx.accounts.initiator_fragment_account.amount;
    require!(initiator_fragments > 0, FragmentError::NoFragmentsHeld);

    let fragments_to_buy = vault_total_fragments
        .checked_sub(initiator_fragments)
        .ok_or(FragmentError::MathOverflow)?;

    let total_buyout_amount = price_per_fragment
        .checked_mul(fragments_to_buy)
        .ok_or(FragmentError::MathOverflow)?;

    require!(
        ctx.accounts.initiator_payment_account.amount >= total_buyout_amount,
        FragmentError::InsufficientPayment
    );

    transfer_fragments_to_escrow(&ctx, initiator_fragments)?;

    transfer_payment_to_escrow(&ctx, total_buyout_amount)?;

    let vault_key = ctx.accounts.vault.key();
    let initiator_key = ctx.accounts.initiator.key();
    let payment_mint_key = ctx.accounts.payment_mint.key();

    let buyout = &mut ctx.accounts.buyout;
    buyout.vault = vault_key;
    buyout.initiator = initiator_key;
    buyout.price_per_fragment = price_per_fragment;
    buyout.total_buyout_amount = total_buyout_amount;
    buyout.fragments_to_buy = fragments_to_buy;
    buyout.fragments_accepted = 0;
    buyout.payment_collected = 0;
    buyout.payment_mint = payment_mint_key;
    buyout.status = BuyoutStatus::Pending;
    buyout.initiated_at = clock.unix_timestamp;
    buyout.expires_at = clock.unix_timestamp
        .checked_add(buyout_duration)
        .ok_or(FragmentError::MathOverflow)?;
    buyout.completed_at = None;
    buyout.bump = ctx.bumps.buyout;

    let vault = &mut ctx.accounts.vault;
    vault.status = VaultStatus::BuyoutPending;

    let config = &mut ctx.accounts.config;
    config.active_buyouts = config.active_buyouts
        .checked_add(1)
        .ok_or(FragmentError::MathOverflow)?;
    config.updated_at = clock.unix_timestamp;

    msg!("Buyout initiated successfully!");
    msg!("Price per fragment: {}", price_per_fragment);
    msg!("Total buyout amount: {}", total_buyout_amount);
    msg!("Fragments to buy: {}", fragments_to_buy);

    Ok(())
}

#[inline(never)]
fn transfer_fragments_to_escrow(ctx: &Context<InitiateBuyout>, amount: u64) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.initiator_fragment_account.to_account_info(),
            to: ctx.accounts.buyout_fragment_escrow.to_account_info(),
            authority: ctx.accounts.initiator.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;
    Ok(())
}

#[inline(never)]
fn transfer_payment_to_escrow(ctx: &Context<InitiateBuyout>, amount: u64) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.initiator_payment_account.to_account_info(),
            to: ctx.accounts.buyout_payment_escrow.to_account_info(),
            authority: ctx.accounts.initiator.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;
    Ok(())
}
