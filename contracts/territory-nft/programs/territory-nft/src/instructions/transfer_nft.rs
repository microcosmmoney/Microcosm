// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::TerritoryError;
use crate::state::{TerritoryCollection, TerritoryNft};

#[derive(Accounts)]
#[instruction(territory_type: u8, territory_id: u64)]
pub struct TransferNft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub new_owner: UncheckedAccount<'info>,

    #[account(
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
    )]
    pub collection_config: Account<'info, TerritoryCollection>,

    #[account(
        mut,
        seeds = [TERRITORY_NFT_SEED, &[territory_type], &territory_id.to_le_bytes()],
        bump = territory_nft.bump,
        constraint = territory_nft.owner == owner.key() @ TerritoryError::NotOwner,
        constraint = territory_nft.can_transfer() @ TerritoryError::CannotBurnWithActiveMembers,
    )]
    pub territory_nft: Account<'info, TerritoryNft>,

    #[account(
        mut,
        constraint = owner_token_account.owner == owner.key(),
        constraint = owner_token_account.mint == territory_nft.mint,
        constraint = owner_token_account.amount == 1,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = new_owner_token_account.owner == new_owner.key(),
        constraint = new_owner_token_account.mint == territory_nft.mint,
    )]
    pub new_owner_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferNft>, _territory_type: u8, _territory_id: u64) -> Result<()> {
    let territory_nft = &mut ctx.accounts.territory_nft;
    let clock = Clock::get()?;

    require!(territory_nft.can_transfer(), TerritoryError::CannotBurnWithActiveMembers);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_token_account.to_account_info(),
                to: ctx.accounts.new_owner_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        1,
    )?;

    let old_owner = territory_nft.owner;
    territory_nft.owner = ctx.accounts.new_owner.key();
    territory_nft.updated_at = clock.unix_timestamp;

    msg!(
        "Territory NFT transferred: type={:?}, id={}, from={}, to={}",
        territory_nft.territory_type,
        territory_nft.territory_id,
        old_owner,
        ctx.accounts.new_owner.key()
    );

    Ok(())
}
