use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentConfig, FragmentVault, VaultStatus};

#[derive(Accounts)]
pub struct TransferFragments<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub recipient: AccountInfo<'info>,

    #[account(
        seeds = [FRAGMENT_CONFIG_SEED],
        bump = config.bump,
        constraint = !config.is_paused @ FragmentError::OperationPaused
    )]
    pub config: Account<'info, FragmentConfig>,

    #[account(
        seeds = [FRAGMENT_VAULT_SEED, vault.nft_mint.as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::Active @ FragmentError::VaultNotActive
    )]
    pub vault: Account<'info, FragmentVault>,

    pub fragment_mint: AccountInfo<'info>,

    #[account(
        mut,
        constraint = sender_fragment_account.owner == sender.key(),
        constraint = sender_fragment_account.mint == vault.fragment_mint @ FragmentError::InvalidFragmentMint
    )]
    pub sender_fragment_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = fragment_mint,
        associated_token::authority = recipient
    )]
    pub recipient_fragment_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<TransferFragments>, amount: u64) -> Result<()> {
    require!(amount > 0, FragmentError::InvalidAmount);
    require!(
        ctx.accounts.sender_fragment_account.amount >= amount,
        FragmentError::InsufficientFragments
    );

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.sender_fragment_account.to_account_info(),
            to: ctx.accounts.recipient_fragment_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;

    msg!("Fragments transferred successfully!");
    msg!("From: {}", ctx.accounts.sender.key());
    msg!("To: {}", ctx.accounts.recipient.key());
    msg!("Amount: {}", amount);

    Ok(())
}
