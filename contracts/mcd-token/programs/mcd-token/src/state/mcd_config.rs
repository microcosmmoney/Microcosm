// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[account]
pub struct McdConfig {
    pub authority: Pubkey,

    pub mcd_mint: Pubkey,

    pub genesis_pool: Pubkey,

    pub recycle_pool: Pubkey,

    pub total_minted: u64,

    pub total_recycled: u64,

    pub total_vault_minted: u64,

    pub total_consumed: u64,

    pub total_usdc_deposited: u64,

    pub total_usdc_withdrawn: u64,

    pub total_burned: u64,

    pub total_fees_collected: u64,

    pub last_update_timestamp: i64,

    pub bump: u8,
}

impl McdConfig {
    pub const LEN: usize = 8 +
        32 +
        32 +
        32 +
        32 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        1;
}

#[account]
pub struct StationMcdVault {
    pub station_id: u64,

    pub balance: u64,

    pub total_received: u64,

    pub total_distributed: u64,

    pub last_distribution_date: i64,

    pub token_account: Pubkey,

    pub bump: u8,

    pub _reserved: [u8; 32],
}

impl StationMcdVault {
    pub const LEN: usize = 8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        32 +
        1 +
        32;
}

#[account]
pub struct UserMcdAccount {
    pub uid: u64,

    pub station_id: u64,

    pub balance: u64,

    pub total_received: u64,

    pub total_spent: u64,

    pub total_deposited: u64,

    pub total_withdrawn: u64,

    pub token_account: Pubkey,

    pub bump: u8,
}

impl UserMcdAccount {
    pub const LEN: usize = 8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        8 +
        32 +
        1;
}
