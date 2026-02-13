use anchor_lang::prelude::*;

#[account]
pub struct McdWhitelist {
    pub project_id: u64,

    pub wallet_address: Pubkey,

    pub project_name: [u8; 64],

    pub registered_at: i64,

    pub status: u8,

    pub updated_at: i64,

    pub bump: u8,
}

impl McdWhitelist {
    pub const LEN: usize = 8 +
        8 +
        32 +
        64 +
        8 +
        1 +
        8 +
        1;

    pub const STATUS_ACTIVE: u8 = 0;

    pub const STATUS_SUSPENDED: u8 = 1;
}
