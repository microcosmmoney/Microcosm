// AI-generated · AI-managed · AI-maintained
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ipYLo2aitVyNXGzCgpiaKKA2JLvtbBvMHiV7qKZTPVZ");

#[program]
pub mod territory_nft {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::initialize_collection::handler_config(ctx)
    }

    pub fn initialize_collection_nft(
        ctx: Context<InitializeCollectionNft>,
        uri: String,
    ) -> Result<()> {
        instructions::initialize_collection::handler_nft(ctx, uri)
    }

    pub fn mint_station_nft_init_pda(
        ctx: Context<MintStationNftInitPda>,
        station_id: u64,
    ) -> Result<()> {
        instructions::mint_station_nft::handler_init_pda(ctx, station_id)
    }

    pub fn mint_station_nft_init_token(
        ctx: Context<MintStationNftInitToken>,
        station_id: u64,
    ) -> Result<()> {
        instructions::mint_station_nft::handler_init_token(ctx, station_id)
    }

    pub fn mint_station_nft_finalize(
        ctx: Context<MintStationNftFinalize>,
        station_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_station_nft::handler_finalize(ctx, station_id, uri)
    }

    #[allow(dead_code)]
    pub fn mint_station_nft_init(
        ctx: Context<MintStationNftInit>,
        station_id: u64,
    ) -> Result<()> {
        instructions::mint_station_nft::handler_init(ctx, station_id)
    }

    pub fn mint_matrix_nft_init_pda(
        ctx: Context<MintMatrixNftInitPda>,
        matrix_id: u64,
        parent_sector_id: u64,
    ) -> Result<()> {
        instructions::mint_matrix_nft_v2::handler_init_pda(ctx, matrix_id, parent_sector_id)
    }

    pub fn mint_matrix_nft_init_token(
        ctx: Context<MintMatrixNftInitToken>,
        matrix_id: u64,
    ) -> Result<()> {
        instructions::mint_matrix_nft_v2::handler_init_token(ctx, matrix_id)
    }

    pub fn mint_matrix_nft_finalize(
        ctx: Context<MintMatrixNftFinalize>,
        matrix_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_matrix_nft_v2::handler_finalize(ctx, matrix_id, uri)
    }

    pub fn mint_sector_nft_init_pda(
        ctx: Context<MintSectorNftInitPda>,
        sector_id: u64,
        parent_system_id: u64,
    ) -> Result<()> {
        instructions::mint_sector_nft_v2::handler_init_pda(ctx, sector_id, parent_system_id)
    }

    pub fn mint_sector_nft_init_token(
        ctx: Context<MintSectorNftInitToken>,
        sector_id: u64,
    ) -> Result<()> {
        instructions::mint_sector_nft_v2::handler_init_token(ctx, sector_id)
    }

    pub fn mint_sector_nft_finalize(
        ctx: Context<MintSectorNftFinalize>,
        sector_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_sector_nft_v2::handler_finalize(ctx, sector_id, uri)
    }

    pub fn mint_system_nft_init_pda(
        ctx: Context<MintSystemNftInitPda>,
        system_id: u64,
    ) -> Result<()> {
        instructions::mint_system_nft_v2::handler_init_pda(ctx, system_id)
    }

    pub fn mint_system_nft_init_token(
        ctx: Context<MintSystemNftInitToken>,
        system_id: u64,
    ) -> Result<()> {
        instructions::mint_system_nft_v2::handler_init_token(ctx, system_id)
    }

    pub fn mint_system_nft_finalize(
        ctx: Context<MintSystemNftFinalize>,
        system_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_system_nft_v2::handler_finalize(ctx, system_id, uri)
    }

    #[allow(dead_code)]
    pub fn mint_matrix_nft(
        ctx: Context<MintMatrixNft>,
        matrix_id: u64,
        parent_sector_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_matrix_nft::handler(ctx, matrix_id, parent_sector_id, uri)
    }

    #[allow(dead_code)]
    pub fn mint_sector_nft(
        ctx: Context<MintSectorNft>,
        sector_id: u64,
        parent_system_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_sector_nft::handler(ctx, sector_id, parent_system_id, uri)
    }

    #[allow(dead_code)]
    pub fn mint_system_nft(
        ctx: Context<MintSystemNft>,
        system_id: u64,
        uri: String,
    ) -> Result<()> {
        instructions::mint_system_nft::handler(ctx, system_id, uri)
    }

    pub fn transfer_nft(
        ctx: Context<TransferNft>,
        territory_type: u8,
        territory_id: u64,
    ) -> Result<()> {
        instructions::transfer_nft::handler(ctx, territory_type, territory_id)
    }

    pub fn burn_nft(
        ctx: Context<BurnNft>,
        territory_type: u8,
        territory_id: u64,
    ) -> Result<()> {
        instructions::burn_nft::handler(ctx, territory_type, territory_id)
    }
}
