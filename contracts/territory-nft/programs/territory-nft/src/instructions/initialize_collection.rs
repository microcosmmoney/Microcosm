// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::constants::*;
use crate::error::TerritoryError;
use crate::state::TerritoryCollection;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = TerritoryCollection::LEN,
        seeds = [COLLECTION_SEED],
        bump
    )]
    pub collection_config: Box<Account<'info, TerritoryCollection>>,

    pub system_program: Program<'info, System>,
}

pub fn handler_config(ctx: Context<InitializeConfig>) -> Result<()> {
    let collection_config = &mut ctx.accounts.collection_config;
    let clock = Clock::get()?;

    require!(
        !collection_config.is_initialized,
        TerritoryError::CollectionAlreadyInitialized
    );

    let bump = ctx.bumps.collection_config;
    collection_config.authority = ctx.accounts.authority.key();
    collection_config.created_at = clock.unix_timestamp;
    collection_config.updated_at = clock.unix_timestamp;
    collection_config.bump = bump;

    msg!("Step 1: Config PDA created");
    msg!("Config PDA: {}", collection_config.key());

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCollectionNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
        constraint = collection_config.authority == authority.key() @ TerritoryError::Unauthorized,
        constraint = !collection_config.is_initialized @ TerritoryError::CollectionAlreadyInitialized
    )]
    pub collection_config: Box<Account<'info, TerritoryCollection>>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = collection_config,
        mint::freeze_authority = collection_config,
    )]
    pub collection_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        token::mint = collection_mint,
        token::authority = authority,
    )]
    pub collection_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub collection_master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
fn mint_collection_nft<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    mint_to(
        CpiContext::new_with_signer(
            token_program,
            MintTo { mint, to, authority },
            signer_seeds,
        ),
        1,
    )
}

#[inline(never)]
fn create_metadata<'info>(
    token_metadata_program: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    mint_authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    uri: String,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let data = DataV2 {
        name: COLLECTION_NAME.to_string(),
        symbol: COLLECTION_SYMBOL.to_string(),
        uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            token_metadata_program,
            CreateMetadataAccountsV3 {
                metadata,
                mint,
                mint_authority,
                payer,
                update_authority,
                system_program,
                rent,
            },
            signer_seeds,
        ),
        data,
        true,
        true,
        None,
    )
}

#[inline(never)]
fn create_edition<'info>(
    token_metadata_program: AccountInfo<'info>,
    edition: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    mint_authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    create_master_edition_v3(
        CpiContext::new_with_signer(
            token_metadata_program,
            CreateMasterEditionV3 {
                edition,
                mint,
                update_authority,
                mint_authority,
                payer,
                metadata,
                token_program,
                system_program,
                rent,
            },
            signer_seeds,
        ),
        Some(0),
    )
}

pub fn handler_nft(ctx: Context<InitializeCollectionNft>, uri: String) -> Result<()> {
    let clock = Clock::get()?;
    let bump = ctx.accounts.collection_config.bump;
    let seeds = &[COLLECTION_SEED, &[bump]];
    let signer_seeds = &[&seeds[..]];

    mint_collection_nft(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.collection_token_account.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        signer_seeds,
    )?;

    create_metadata(
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        uri,
        signer_seeds,
    )?;

    create_edition(
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.collection_master_edition.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        signer_seeds,
    )?;

    let collection_config = &mut ctx.accounts.collection_config;
    collection_config.collection_mint = ctx.accounts.collection_mint.key();
    collection_config.collection_metadata = ctx.accounts.collection_metadata.key();
    collection_config.collection_master_edition = ctx.accounts.collection_master_edition.key();
    collection_config.total_stations = 0;
    collection_config.total_matrices = 0;
    collection_config.total_sectors = 0;
    collection_config.total_systems = 0;
    collection_config.updated_at = clock.unix_timestamp;
    collection_config.is_initialized = true;

    msg!("Step 2: Collection NFT created");
    msg!("Collection Mint: {}", ctx.accounts.collection_mint.key());

    Ok(())
}
