// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};

use crate::constants::*;
use crate::error::TerritoryError;
use crate::state::{TerritoryCollection, TerritoryNft, TerritoryType};

#[derive(Accounts)]
#[instruction(territory_type: u8, territory_id: u64)]
pub struct BurnNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
        constraint = collection_config.authority == authority.key() @ TerritoryError::Unauthorized
    )]
    pub collection_config: Account<'info, TerritoryCollection>,

    #[account(
        mut,
        seeds = [TERRITORY_NFT_SEED, &[territory_type], &territory_id.to_le_bytes()],
        bump = territory_nft.bump,
        close = authority,
    )]
    pub territory_nft: Account<'info, TerritoryNft>,

    #[account(
        mut,
        constraint = nft_mint.key() == territory_nft.mint,
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = nft_token_account.mint == territory_nft.mint,
        constraint = nft_token_account.amount == 1,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    pub token_account_owner: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<BurnNft>, territory_type: u8, territory_id: u64) -> Result<()> {
    let collection_config = &mut ctx.accounts.collection_config;
    let territory_nft = &ctx.accounts.territory_nft;
    let clock = Clock::get()?;

    require!(territory_nft.can_burn(), TerritoryError::CannotBurnWithActiveMembers);

    let bump = collection_config.bump;
    let seeds = &[COLLECTION_SEED, &[bump]];
    let signer_seeds = &[&seeds[..]];

    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nft_mint.to_account_info(),
                from: ctx.accounts.nft_token_account.to_account_info(),
                authority: collection_config.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    let t_type = TerritoryType::from_u8(territory_type).ok_or(TerritoryError::InvalidTerritoryType)?;
    match t_type {
        TerritoryType::Station => {
            collection_config.total_stations = collection_config.total_stations.saturating_sub(1);
        }
        TerritoryType::Matrix => {
            collection_config.total_matrices = collection_config.total_matrices.saturating_sub(1);
        }
        TerritoryType::Sector => {
            collection_config.total_sectors = collection_config.total_sectors.saturating_sub(1);
        }
        TerritoryType::System => {
            collection_config.total_systems = collection_config.total_systems.saturating_sub(1);
        }
    }
    collection_config.updated_at = clock.unix_timestamp;

    msg!(
        "Territory NFT burned: type={:?}, id={}, mint={}",
        t_type,
        territory_id,
        ctx.accounts.nft_mint.key()
    );

    Ok(())
}
