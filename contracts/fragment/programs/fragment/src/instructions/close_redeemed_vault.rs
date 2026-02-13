use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::constants::*;
use crate::error::FragmentError;
use crate::state::{FragmentVault, VaultStatus};

#[derive(Accounts)]
pub struct CloseRedeemedVault<'info> {
    #[account(mut)]
    pub original_owner: Signer<'info>,

    #[account(
        mut,
        seeds = [FRAGMENT_VAULT_SEED, nft_mint.key().as_ref()],
        bump = vault.bump,
        constraint = vault.status == VaultStatus::Redeemed @ FragmentError::VaultNotRedeemed,
        constraint = vault.original_owner == original_owner.key() @ FragmentError::NotOriginalOwner,
        close = original_owner
    )]
    pub vault: Account<'info, FragmentVault>,

    pub nft_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseRedeemedVault>) -> Result<()> {
    msg!("Closing redeemed vault");
    msg!("NFT Mint: {}", ctx.accounts.nft_mint.key());
    msg!("Original Owner: {}", ctx.accounts.original_owner.key());
    msg!("Rent returned to original owner");

    Ok(())
}
