// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[account]
pub struct VerifierConfig {
    pub authority: Pubkey,

    pub territory_nft_program: Pubkey,

    pub mcc_token_program: Pubkey,

    pub total_users: u64,

    pub level_1_count: u64,
    pub level_2_count: u64,
    pub level_3_count: u64,
    pub level_4_count: u64,
    pub level_5_count: u64,
    pub level_6_count: u64,
    pub level_7_count: u64,

    pub is_paused: bool,

    pub is_initialized: bool,

    pub created_at: i64,

    pub updated_at: i64,

    pub _reserved: [u8; 64],
}

impl VerifierConfig {
    pub const SPACE: usize = 8 +
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
        1 +
        1 +
        8 +
        8 +
        64;

    pub fn increment_users(&mut self) {
        self.total_users = self.total_users.saturating_add(1);
        self.level_1_count = self.level_1_count.saturating_add(1);
    }

    pub fn update_level_stats(&mut self, from_level: u8, to_level: u8) {
        match from_level {
            1 => self.level_1_count = self.level_1_count.saturating_sub(1),
            2 => self.level_2_count = self.level_2_count.saturating_sub(1),
            3 => self.level_3_count = self.level_3_count.saturating_sub(1),
            4 => self.level_4_count = self.level_4_count.saturating_sub(1),
            5 => self.level_5_count = self.level_5_count.saturating_sub(1),
            6 => self.level_6_count = self.level_6_count.saturating_sub(1),
            7 => self.level_7_count = self.level_7_count.saturating_sub(1),
            _ => {}
        }

        match to_level {
            1 => self.level_1_count = self.level_1_count.saturating_add(1),
            2 => self.level_2_count = self.level_2_count.saturating_add(1),
            3 => self.level_3_count = self.level_3_count.saturating_add(1),
            4 => self.level_4_count = self.level_4_count.saturating_add(1),
            5 => self.level_5_count = self.level_5_count.saturating_add(1),
            6 => self.level_6_count = self.level_6_count.saturating_add(1),
            7 => self.level_7_count = self.level_7_count.saturating_add(1),
            _ => {}
        }
    }
}

impl Default for VerifierConfig {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            territory_nft_program: Pubkey::default(),
            mcc_token_program: Pubkey::default(),
            total_users: 0,
            level_1_count: 0,
            level_2_count: 0,
            level_3_count: 0,
            level_4_count: 0,
            level_5_count: 0,
            level_6_count: 0,
            level_7_count: 0,
            is_paused: false,
            is_initialized: false,
            created_at: 0,
            updated_at: 0,
            _reserved: [0u8; 64],
        }
    }
}
