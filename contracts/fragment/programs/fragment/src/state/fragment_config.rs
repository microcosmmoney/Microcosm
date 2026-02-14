// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct FragmentConfig {
    pub authority: Pubkey,

    pub min_fragments: u64,

    pub max_fragments: u64,

    pub buyout_duration: i64,

    pub total_fragmented_nfts: u64,

    pub active_buyouts: u64,

    pub is_paused: bool,

    pub created_at: i64,

    pub updated_at: i64,

    pub bump: u8,
}

impl FragmentConfig {
    pub const LEN: usize = 8 +
        32 +
        8 +
        8 +
        8 +
        8 +
        8 +
        1 +
        8 +
        8 +
        1;

    pub fn initialize_defaults(&mut self) {
        self.min_fragments = MIN_FRAGMENT_COUNT;
        self.max_fragments = MAX_FRAGMENT_COUNT;
        self.buyout_duration = DEFAULT_BUYOUT_DURATION;
        self.total_fragmented_nfts = 0;
        self.active_buyouts = 0;
        self.is_paused = false;
    }

    pub fn validate_fragment_count(&self, count: u64) -> bool {
        count >= self.min_fragments && count <= self.max_fragments
    }
}
