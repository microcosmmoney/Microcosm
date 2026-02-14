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
#[instruction(sector_id: u64, parent_system_id: u64)]
pub struct MintSectorNftInitPda<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub recipient: UncheckedAccount<'info>,

    #[account(
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
        constraint = collection_config.authority == authority.key() @ TerritoryError::Unauthorized
    )]
    pub collection_config: Box<Account<'info, TerritoryCollection>>,

    #[account(
        init,
        payer = authority,
        space = TerritoryNft::LEN,
        seeds = [TERRITORY_NFT_SEED, &[TerritoryType::Sector as u8], &sector_id.to_le_bytes()],
        bump
    )]
    pub territory_nft: Box<Account<'info, TerritoryNft>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sector_id: u64)]
pub struct MintSectorNftInitToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub recipient: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
        constraint = collection_config.authority == authority.key() @ TerritoryError::Unauthorized
    )]
    pub collection_config: Box<Account<'info, TerritoryCollection>>,

    #[account(
        mut,
        seeds = [TERRITORY_NFT_SEED, &[TerritoryType::Sector as u8], &sector_id.to_le_bytes()],
        bump = territory_nft.bump
    )]
    pub territory_nft: Box<Account<'info, TerritoryNft>>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = collection_config,
        mint::freeze_authority = collection_config,
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = nft_mint,
        associated_token::authority = recipient,
    )]
    pub nft_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(sector_id: u64)]
pub struct MintSectorNftFinalize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [COLLECTION_SEED],
        bump = collection_config.bump,
        constraint = collection_config.authority == authority.key() @ TerritoryError::Unauthorized
    )]
    pub collection_config: Box<Account<'info, TerritoryCollection>>,

    #[account(
        mut,
        seeds = [TERRITORY_NFT_SEED, &[TerritoryType::Sector as u8], &sector_id.to_le_bytes()],
        bump = territory_nft.bump
    )]
    pub territory_nft: Box<Account<'info, TerritoryNft>>,

    #[account(
        mut,
        constraint = nft_mint.key() == territory_nft.mint @ TerritoryError::InvalidParameter
    )]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub nft_master_edition: UncheckedAccount<'info>,

    #[account(
        constraint = collection_mint.key() == collection_config.collection_mint @ TerritoryError::InvalidCollection
    )]
    pub collection_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    pub collection_master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
fn mint_sector_token<'info>(
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
fn create_sector_metadata<'info>(
    token_metadata_program: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    mint_authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    collection_mint_key: Pubkey,
    sector_id: u64,
    uri: String,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let name = format!("{}{}", SECTOR_NAME_PREFIX, sector_id);
    let data = DataV2 {
        name,
        symbol: NFT_SYMBOL.to_string(),
        uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: Some(Collection {
            verified: false,
            key: collection_mint_key,
        }),
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
fn create_sector_edition<'info>(
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

#[inline(never)]
fn verify_sector_collection<'info>(
    token_metadata_program: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    collection_authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    collection_mint: AccountInfo<'info>,
    collection_metadata: AccountInfo<'info>,
    collection_master_edition: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    set_and_verify_collection(
        CpiContext::new_with_signer(
            token_metadata_program,
            SetAndVerifyCollection {
                metadata,
                collection_authority,
                payer,
                update_authority,
                collection_mint,
                collection_metadata,
                collection_master_edition,
            },
            signer_seeds,
        ),
        None,
    )
}

pub fn handler_init_pda(ctx: Context<MintSectorNftInitPda>, sector_id: u64, parent_system_id: u64) -> Result<()> {
    let clock = Clock::get()?;

    require!(sector_id > 0, TerritoryError::InvalidTerritoryId);

    let recipient_key = ctx.accounts.recipient.key();

    let territory_nft = &mut ctx.accounts.territory_nft;
    territory_nft.territory_type = TerritoryType::Sector;
    territory_nft.territory_id = sector_id;
    territory_nft.mint = Pubkey::default();
    territory_nft.owner = recipient_key;
    territory_nft.parent_id = parent_system_id;
    territory_nft.current_members = 0;
    territory_nft.is_auctioning = false;
    territory_nft.is_frozen = false;
    territory_nft.created_at = clock.unix_timestamp;
    territory_nft.updated_at = clock.unix_timestamp;
    territory_nft.bump = ctx.bumps.territory_nft;

    msg!("Step 1: Sector NFT PDA created: sector_id={}, parent_system_id={}", sector_id, parent_system_id);

    Ok(())
}

pub fn handler_init_token(ctx: Context<MintSectorNftInitToken>, sector_id: u64) -> Result<()> {
    let clock = Clock::get()?;

    let bump = ctx.accounts.collection_config.bump;
    let seeds = &[COLLECTION_SEED, &[bump]];
    let signer_seeds = &[&seeds[..]];

    let nft_mint_key = ctx.accounts.nft_mint.key();

    mint_sector_token(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.nft_mint.to_account_info(),
        ctx.accounts.nft_token_account.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        signer_seeds,
    )?;

    let territory_nft = &mut ctx.accounts.territory_nft;
    territory_nft.mint = nft_mint_key;
    territory_nft.updated_at = clock.unix_timestamp;

    msg!("Step 2: Sector Token created and minted: sector_id={}, mint={}", sector_id, nft_mint_key);

    Ok(())
}

pub fn handler_finalize(ctx: Context<MintSectorNftFinalize>, sector_id: u64, uri: String) -> Result<()> {
    let clock = Clock::get()?;

    let bump = ctx.accounts.collection_config.bump;
    let seeds = &[COLLECTION_SEED, &[bump]];
    let signer_seeds = &[&seeds[..]];

    let collection_mint_key = ctx.accounts.collection_mint.key();
    let nft_mint_key = ctx.accounts.nft_mint.key();

    create_sector_metadata(
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.nft_metadata.to_account_info(),
        ctx.accounts.nft_mint.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        collection_mint_key,
        sector_id,
        uri,
        signer_seeds,
    )?;

    create_sector_edition(
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.nft_master_edition.to_account_info(),
        ctx.accounts.nft_mint.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.nft_metadata.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        signer_seeds,
    )?;

    verify_sector_collection(
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.nft_metadata.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.collection_config.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_master_edition.to_account_info(),
        signer_seeds,
    )?;

    let collection_config = &mut ctx.accounts.collection_config;
    collection_config.total_sectors = collection_config.total_sectors.saturating_add(1);
    collection_config.updated_at = clock.unix_timestamp;

    msg!("Step 3: Sector NFT finalized: sector_id={}, mint={}", sector_id, nft_mint_key);

    Ok(())
}
