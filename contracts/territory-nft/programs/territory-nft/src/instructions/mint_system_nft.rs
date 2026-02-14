// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::types::{Collection, DataV2},
        set_and_verify_collection,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
        SetAndVerifyCollection,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::constants::*;
use crate::error::TerritoryError;
use crate::state::{TerritoryCollection, TerritoryNft, TerritoryType};

#[derive(Accounts)]
#[instruction(system_id: u64)]
pub struct MintSystemNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub recipient: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
        constraint = collection_config.authority == authority.key() @ TerritoryError::Unauthorized
    )]
    pub collection_config: Account<'info, TerritoryCollection>,

    #[account(
        init,
        payer = authority,
        space = TerritoryNft::LEN,
        seeds = [TERRITORY_NFT_SEED, &[TerritoryType::System as u8], &system_id.to_le_bytes()],
        bump
    )]
    pub territory_nft: Account<'info, TerritoryNft>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = collection_config,
        mint::freeze_authority = collection_config,
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = nft_mint,
        associated_token::authority = recipient,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub nft_master_edition: UncheckedAccount<'info>,

    #[account(constraint = collection_mint.key() == collection_config.collection_mint)]
    pub collection_mint: Account<'info, Mint>,

    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    pub collection_master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<MintSystemNft>, system_id: u64, uri: String) -> Result<()> {
    let collection_config = &mut ctx.accounts.collection_config;
    let territory_nft = &mut ctx.accounts.territory_nft;
    let clock = Clock::get()?;

    require!(system_id > 0, TerritoryError::InvalidTerritoryId);

    let bump = collection_config.bump;
    let seeds = &[COLLECTION_SEED, &[bump]];
    let signer_seeds = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint.to_account_info(),
                to: ctx.accounts.nft_token_account.to_account_info(),
                authority: collection_config.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    let name = format!("{}{}", SYSTEM_NAME_PREFIX, system_id);

    let data = DataV2 {
        name,
        symbol: NFT_SYMBOL.to_string(),
        uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: Some(Collection {
            verified: false,
            key: ctx.accounts.collection_mint.key(),
        }),
        uses: None,
    };

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                mint_authority: collection_config.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                update_authority: collection_config.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        data,
        true,
        true,
        None,
    )?;

    create_master_edition_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.nft_master_edition.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                update_authority: collection_config.to_account_info(),
                mint_authority: collection_config.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        Some(0),
    )?;

    set_and_verify_collection(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            SetAndVerifyCollection {
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                collection_authority: collection_config.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                update_authority: collection_config.to_account_info(),
                collection_mint: ctx.accounts.collection_mint.to_account_info(),
                collection_metadata: ctx.accounts.collection_metadata.to_account_info(),
                collection_master_edition: ctx.accounts.collection_master_edition.to_account_info(),
            },
            signer_seeds,
        ),
        None,
    )?;

    territory_nft.territory_type = TerritoryType::System;
    territory_nft.territory_id = system_id;
    territory_nft.mint = ctx.accounts.nft_mint.key();
    territory_nft.owner = ctx.accounts.recipient.key();
    territory_nft.parent_id = 0;
    territory_nft.current_members = 0;
    territory_nft.is_auctioning = false;
    territory_nft.is_frozen = false;
    territory_nft.created_at = clock.unix_timestamp;
    territory_nft.updated_at = clock.unix_timestamp;
    territory_nft.bump = ctx.bumps.territory_nft;

    collection_config.total_systems = collection_config.total_systems.saturating_add(1);
    collection_config.updated_at = clock.unix_timestamp;

    msg!("System NFT minted: system_id={}, mint={}", system_id, ctx.accounts.nft_mint.key());

    Ok(())
}
