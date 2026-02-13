use anchor_lang::prelude::*;

#[account]
pub struct TerritoryCollection {
    pub authority: Pubkey,

    pub collection_mint: Pubkey,

    pub collection_metadata: Pubkey,

    pub collection_master_edition: Pubkey,

    pub total_stations: u64,

    pub total_matrices: u64,

    pub total_sectors: u64,

    pub total_systems: u64,

    pub created_at: i64,

    pub updated_at: i64,

    pub is_initialized: bool,

    pub bump: u8,

    pub _reserved: [u8; 64],
}

impl TerritoryCollection {
    pub const LEN: usize = 8
        + 32
        + 32
        + 32
        + 32
        + 8
        + 8
        + 8
        + 8
        + 8
        + 8
        + 1
        + 1
        + 64;

    pub fn total_nfts(&self) -> u64 {
        self.total_stations
            .saturating_add(self.total_matrices)
            .saturating_add(self.total_sectors)
            .saturating_add(self.total_systems)
    }
}

impl Default for TerritoryCollection {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            collection_mint: Pubkey::default(),
            collection_metadata: Pubkey::default(),
            collection_master_edition: Pubkey::default(),
            total_stations: 0,
            total_matrices: 0,
            total_sectors: 0,
            total_systems: 0,
            created_at: 0,
            updated_at: 0,
            is_initialized: false,
            bump: 0,
            _reserved: [0u8; 64],
        }
    }
}
