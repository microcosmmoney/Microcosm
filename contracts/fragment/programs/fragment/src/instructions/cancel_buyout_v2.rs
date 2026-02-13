use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentConfig, FragmentVault, Buyout, BuyoutStatus, VaultStatus};

#[derive(Accounts)]
pub struct CancelBuyoutRefund<'info> {
    #[account(mut)]
    pub canceller: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, vault.nft_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::BuyoutPending @ FragmentError::NoBuyoutInProgress
    )]
    pub vault: Box<Account<'info, FragmentVault>>,

    #[account(
        mut,
        seeds = [BUYOUT_SEED, vault.key().as_ref()],
        bump = buyout.bump,
        constraint = buyout.status == BuyoutStatus::Pending @ FragmentError::BuyoutNotPending
    )]
    pub buyout: Box<Account<'info, Buyout>>,

    #[account(
        mut,
        constraint = initiator.key() == buyout.initiator @ FragmentError::InvalidInitiator
    )]
    pub initiator: AccountInfo<'info>,

    #[account(
        seeds = [FRAGMENT_MINT_SEED, vault.nft_mint.as_ref()],
        bump
    )]
    pub fragment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = fragment_mint,
        associated_token::authority = buyout
    )]
    pub buyout_fragment_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = canceller,
        associated_token::mint = fragment_mint,
        associated_token::authority = initiator
    )]
    pub initiator_fragment_account: Box<Account<'info, TokenAccount>>,

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
        payer = canceller,
        associated_token::mint = payment_mint,
        associated_token::authority = initiator
    )]
    pub initiator_payment_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_refund(ctx: Context<CancelBuyoutRefund>) -> Result<()> {
    let clock = Clock::get()?;
    let buyout = &ctx.accounts.buyout;

    let is_initiator = ctx.accounts.canceller.key() == buyout.initiator;
    let is_expired = clock.unix_timestamp >= buyout.expires_at;

    require!(
        is_initiator || is_expired,
        FragmentError::CannotCancelBuyout
    );

    let escrowed_fragments = ctx.accounts.buyout_fragment_escrow.amount;
    if escrowed_fragments > 0 {
        refund_fragments(&ctx, escrowed_fragments)?;
    }

    let remaining_payment = ctx.accounts.buyout_payment_escrow.amount;
    if remaining_payment > 0 {
        refund_payment(&ctx, remaining_payment)?;
    }

    let buyout = &mut ctx.accounts.buyout;
    buyout.status = if is_expired {
        BuyoutStatus::Expired
    } else {
        BuyoutStatus::Cancelled
    };

    msg!("Buyout refund completed (Step 1/2)");
    msg!("Reason: {}", if is_expired { "Expired" } else { "Initiator cancelled" });
    msg!("Fragments returned: {}", escrowed_fragments);
    msg!("Payment returned: {}", remaining_payment);

    Ok(())
}

#[inline(never)]
fn refund_fragments(ctx: &Context<CancelBuyoutRefund>, amount: u64) -> Result<()> {
    let vault_key = ctx.accounts.vault.key();
    let buyout_seeds = &[
        BUYOUT_SEED,
        vault_key.as_ref(),
        &[ctx.accounts.buyout.bump],
    ];
    let signer_seeds = &[&buyout_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyout_fragment_escrow.to_account_info(),
            to: ctx.accounts.initiator_fragment_account.to_account_info(),
            authority: ctx.accounts.buyout.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, amount)?;
    Ok(())
}

#[inline(never)]
fn refund_payment(ctx: &Context<CancelBuyoutRefund>, amount: u64) -> Result<()> {
    let vault_key = ctx.accounts.vault.key();
    let buyout_seeds = &[
        BUYOUT_SEED,
        vault_key.as_ref(),
        &[ctx.accounts.buyout.bump],
    ];
    let signer_seeds = &[&buyout_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyout_payment_escrow.to_account_info(),
            to: ctx.accounts.initiator_payment_account.to_account_info(),
            authority: ctx.accounts.buyout.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct CancelBuyoutClose<'info> {
    #[account(mut)]
    pub canceller: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Box<Account<'info, FragmentConfig>>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, vault.nft_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::BuyoutPending @ FragmentError::NoBuyoutInProgress
    )]
    pub vault: Box<Account<'info, FragmentVault>>,

    #[account(
        mut,
        seeds = [BUYOUT_SEED, vault.key().as_ref()],
        bump = buyout.bump,
        constraint = buyout.status == BuyoutStatus::Cancelled || buyout.status == BuyoutStatus::Expired @ FragmentError::BuyoutNotCancelled
    )]
    pub buyout: Box<Account<'info, Buyout>>,

    #[account(
        mut,
        constraint = initiator.key() == buyout.initiator @ FragmentError::InvalidInitiator
    )]
    pub initiator: AccountInfo<'info>,

    #[account(
        seeds = [FRAGMENT_MINT_SEED, vault.nft_mint.as_ref()],
        bump
    )]
    pub fragment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = fragment_mint,
        associated_token::authority = buyout,
        constraint = buyout_fragment_escrow.amount == 0 @ FragmentError::EscrowNotEmpty
    )]
    pub buyout_fragment_escrow: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = payment_mint.key() == buyout.payment_mint @ FragmentError::InvalidPaymentMint
    )]
    pub payment_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = payment_mint,
        associated_token::authority = buyout,
        constraint = buyout_payment_escrow.amount == 0 @ FragmentError::EscrowNotEmpty
    )]
    pub buyout_payment_escrow: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

pub fn handler_close(ctx: Context<CancelBuyoutClose>) -> Result<()> {
    let clock = Clock::get()?;

    close_fragment_escrow(&ctx)?;
    close_payment_escrow(&ctx)?;

    let buyout = &mut ctx.accounts.buyout;
    buyout.completed_at = Some(clock.unix_timestamp);

    let vault = &mut ctx.accounts.vault;
    vault.status = VaultStatus::Active;

    let config = &mut ctx.accounts.config;
    config.active_buyouts = config.active_buyouts
        .checked_sub(1)
        .ok_or(FragmentError::MathOverflow)?;
    config.updated_at = clock.unix_timestamp;

    msg!("Buyout cancelled successfully! (Step 2/2)");
    msg!("Vault status restored to Active");

    Ok(())
}

#[inline(never)]
fn close_fragment_escrow(ctx: &Context<CancelBuyoutClose>) -> Result<()> {
    let vault_key = ctx.accounts.vault.key();
    let buyout_seeds = &[
        BUYOUT_SEED,
        vault_key.as_ref(),
        &[ctx.accounts.buyout.bump],
    ];
    let signer_seeds = &[&buyout_seeds[..]];

    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.buyout_fragment_escrow.to_account_info(),
            destination: ctx.accounts.initiator.to_account_info(),
            authority: ctx.accounts.buyout.to_account_info(),
        },
        signer_seeds,
    );
    token::close_account(close_ctx)?;
    Ok(())
}

#[inline(never)]
fn close_payment_escrow(ctx: &Context<CancelBuyoutClose>) -> Result<()> {
    let vault_key = ctx.accounts.vault.key();
    let buyout_seeds = &[
        BUYOUT_SEED,
        vault_key.as_ref(),
        &[ctx.accounts.buyout.bump],
    ];
    let signer_seeds = &[&buyout_seeds[..]];

    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.buyout_payment_escrow.to_account_info(),
            destination: ctx.accounts.initiator.to_account_info(),
            authority: ctx.accounts.buyout.to_account_info(),
        },
        signer_seeds,
    );
    token::close_account(close_ctx)?;
    Ok(())
}
