// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[error_code]
pub enum TerritoryError {
    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Collection already initialized")]
    CollectionAlreadyInitialized,

    #[msg("Invalid territory type")]
    InvalidTerritoryType,

    #[msg("Territory ID already exists")]
    TerritoryAlreadyExists,

    #[msg("Territory not found")]
    TerritoryNotFound,

    #[msg("Invalid territory ID")]
    InvalidTerritoryId,

    #[msg("Cannot burn NFT with active members")]
    CannotBurnWithActiveMembers,

    #[msg("NFT not owned by caller")]
    NotOwner,

    #[msg("Invalid metadata URI")]
    InvalidMetadataUri,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Invalid collection")]
    InvalidCollection,

    #[msg("NFT already minted for this territory")]
    NftAlreadyMinted,

    #[msg("Parent territory required for this type")]
    ParentTerritoryRequired,

    #[msg("Invalid parent territory")]
    InvalidParentTerritory,

    #[msg("Invalid parameter")]
    InvalidParameter,
}
