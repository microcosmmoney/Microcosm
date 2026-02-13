use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TerritoryType {
    Station = 0,
    Matrix = 1,
    Sector = 2,
    System = 3,
}

impl Default for TerritoryType {
    fn default() -> Self {
        TerritoryType::Station
    }
}

impl TerritoryType {
    pub fn name(&self) -> &'static str {
        match self {
            TerritoryType::Station => "Station",
            TerritoryType::Matrix => "Matrix",
            TerritoryType::Sector => "Sector",
            TerritoryType::System => "System",
        }
    }

    pub fn capacity(&self) -> u32 {
        match self {
            TerritoryType::Station => 1_000,
            TerritoryType::Matrix => 10_000,
            TerritoryType::Sector => 100_000,
            TerritoryType::System => 1_000_000,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TerritoryType::Station),
            1 => Some(TerritoryType::Matrix),
            2 => Some(TerritoryType::Sector),
            3 => Some(TerritoryType::System),
            _ => None,
        }
    }
}

#[account]
pub struct TerritoryNft {
    pub territory_type: TerritoryType,

    pub territory_id: u64,

    pub mint: Pubkey,

    pub owner: Pubkey,

    pub parent_id: u64,

    pub current_members: u32,

    pub is_auctioning: bool,

    pub is_frozen: bool,

    pub created_at: i64,

    pub updated_at: i64,

    pub bump: u8,

    pub _reserved: [u8; 64],
}

impl TerritoryNft {
    pub const LEN: usize = 8
        + 1
        + 8
        + 32
        + 32
        + 8
        + 4
        + 1
        + 1
        + 8
        + 8
        + 1
        + 64;

    pub fn can_burn(&self) -> bool {
        if self.territory_type == TerritoryType::Station {
            return self.current_members == 0;
        }
        false
    }

    pub fn can_transfer(&self) -> bool {
        !self.is_frozen && !self.is_auctioning
    }
}

impl Default for TerritoryNft {
    fn default() -> Self {
        Self {
            territory_type: TerritoryType::default(),
            territory_id: 0,
            mint: Pubkey::default(),
            owner: Pubkey::default(),
            parent_id: 0,
            current_members: 0,
            is_auctioning: false,
            is_frozen: false,
            created_at: 0,
            updated_at: 0,
            bump: 0,
            _reserved: [0u8; 64],
        }
    }
}
