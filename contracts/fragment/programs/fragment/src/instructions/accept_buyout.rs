// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentVault, Buyout, BuyoutStatus, VaultStatus};

#[derive(Accounts)]
pub struct AcceptBuyout<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds = [FRAGMENT_VAULT_SEED, vault.nft_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::BuyoutPending @ FragmentError::NoBuyoutInProgress
    )]
    pub vault: Box<Account<'info, FragmentVault>>,

    #[account(
        mut,
        seeds = [BUYOUT_SEED, vault.key().as_ref()],
        bump = buyout.bump,
        constraint = buyout.status == BuyoutStatus::Pending @ FragmentError::BuyoutNotPending,
        constraint = buyout.initiator != seller.key() @ FragmentError::CannotAcceptOwnBuyout
    )]
    pub buyout: Box<Account<'info, Buyout>>,

    #[account(
        seeds = [FRAGMENT_MINT_SEED, vault.nft_mint.as_ref()],
        bump
    )]
    pub fragment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = seller_fragment_account.owner == seller.key(),
        constraint = seller_fragment_account.mint == fragment_mint.key()
    )]
    pub seller_fragment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = fragment_mint,
        associated_token::authority = buyout
    )]
    pub buyout_fragment_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = payment_mint.key() == buyout.payment_mint @ FragmentError::InvalidPaymentMint
    )]
    pub payment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = payment_mint,
        associated_token::authority = buyout
    )]
    pub buyout_payment_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = payment_mint,
        associated_token::authority = seller
    )]
    pub seller_payment_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<AcceptBuyout>, fragment_amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let buyout_expires_at = ctx.accounts.buyout.expires_at;
    let buyout_fragments_to_buy = ctx.accounts.buyout.fragments_to_buy;
    let buyout_fragments_accepted = ctx.accounts.buyout.fragments_accepted;
    let buyout_price_per_fragment = ctx.accounts.buyout.price_per_fragment;
    let buyout_bump = ctx.accounts.buyout.bump;
    let vault_key = ctx.accounts.vault.key();

    require!(
        clock.unix_timestamp < buyout_expires_at,
        FragmentError::BuyoutExpired
    );

    require!(
        ctx.accounts.seller_fragment_account.amount >= fragment_amount,
        FragmentError::InsufficientFragments
    );

    let remaining_to_buy = buyout_fragments_to_buy
        .checked_sub(buyout_fragments_accepted)
        .ok_or(FragmentError::MathOverflow)?;

    let actual_amount = fragment_amount.min(remaining_to_buy);
    require!(actual_amount > 0, FragmentError::BuyoutAlreadyComplete);

    let payment_amount = buyout_price_per_fragment
        .checked_mul(actual_amount)
        .ok_or(FragmentError::MathOverflow)?;

    transfer_seller_fragments(&ctx, actual_amount)?;

    transfer_payment_to_seller(&ctx, payment_amount, vault_key, buyout_bump)?;

    let buyout = &mut ctx.accounts.buyout;
    buyout.fragments_accepted = buyout.fragments_accepted
        .checked_add(actual_amount)
        .ok_or(FragmentError::MathOverflow)?;
    buyout.payment_collected = buyout.payment_collected
        .checked_add(payment_amount)
        .ok_or(FragmentError::MathOverflow)?;

    msg!("Buyout accepted!");
    msg!("Fragments sold: {}", actual_amount);
    msg!("Payment received: {}", payment_amount);
    msg!("Total fragments accepted: {}/{}", buyout.fragments_accepted, buyout.fragments_to_buy);

    Ok(())
}

#[inline(never)]
fn transfer_seller_fragments(ctx: &Context<AcceptBuyout>, amount: u64) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.seller_fragment_account.to_account_info(),
            to: ctx.accounts.buyout_fragment_escrow.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;
    Ok(())
}

#[inline(never)]
fn transfer_payment_to_seller(
    ctx: &Context<AcceptBuyout>,
    amount: u64,
    vault_key: Pubkey,
    buyout_bump: u8,
) -> Result<()> {
    let buyout_seeds = &[
        BUYOUT_SEED,
        vault_key.as_ref(),
        &[buyout_bump],
    ];
    let signer_seeds = &[&buyout_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyout_payment_escrow.to_account_info(),
            to: ctx.accounts.seller_payment_account.to_account_info(),
            authority: ctx.accounts.buyout.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, amount)?;
    Ok(())
}
