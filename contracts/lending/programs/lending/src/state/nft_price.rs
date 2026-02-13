use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::CollateralType;

#[account]
pub struct NftPriceOracle {
    pub authority: Pubkey,

    pub station_value: u64,

    pub matrix_value: u64,

    pub sector_value: u64,

    pub system_value: u64,

    pub last_update: i64,

    pub created_at: i64,

    pub bump: u8,
}

impl NftPriceOracle {
    pub const LEN: usize = 8 +
        32 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        1;

    pub fn get_value(&self, collateral_type: &CollateralType) -> u64 {
        match collateral_type {
            CollateralType::Station => self.station_value,
            CollateralType::Matrix => self.matrix_value,
            CollateralType::Sector => self.sector_value,
            CollateralType::System => self.system_value,
        }
    }

    pub fn set_value(&mut self, collateral_type: &CollateralType, value: u64) {
        match collateral_type {
            CollateralType::Station => self.station_value = value,
            CollateralType::Matrix => self.matrix_value = value,
            CollateralType::Sector => self.sector_value = value,
            CollateralType::System => self.system_value = value,
        }
    }

    pub fn initialize_default_values(&mut self) {
        self.station_value = DEFAULT_STATION_VALUE_MCC;
        self.matrix_value = DEFAULT_MATRIX_VALUE_MCC;
        self.sector_value = DEFAULT_SECTOR_VALUE_MCC;
        self.system_value = DEFAULT_SYSTEM_VALUE_MCC;
    }
}
