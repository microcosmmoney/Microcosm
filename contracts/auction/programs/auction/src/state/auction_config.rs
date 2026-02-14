// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[account]
pub struct AuctionConfig {
    pub authority: Pubkey,

    pub team_wallet: Pubkey,

    pub mcc_mint: Pubkey,

    pub territory_nft_program: Pubkey,

    pub min_bid_increment_percent: u8,

    pub auction_duration: i64,

    pub extension_duration: i64,

    pub extension_threshold: i64,

    pub next_auction_id: u64,

    pub total_auctions: u64,

    pub total_completed: u64,

    pub total_cancelled: u64,

    pub total_volume: u64,

    pub is_paused: bool,

    pub is_initialized: bool,

    pub created_at: i64,

    pub updated_at: i64,

    pub _reserved: [u8; 64],
}

impl AuctionConfig {
    pub const SPACE: usize = 8 +
        32 +
        32 +
        32 +
        32 +
        1 +
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

    pub fn generate_auction_id(&mut self) -> u64 {
        let id = self.next_auction_id;
        self.next_auction_id = self.next_auction_id.saturating_add(1);
        self.total_auctions = self.total_auctions.saturating_add(1);
        id
    }

    pub fn record_completion(&mut self, amount: u64) {
        self.total_completed = self.total_completed.saturating_add(1);
        self.total_volume = self.total_volume.saturating_add(amount);
    }

    pub fn record_cancellation(&mut self) {
        self.total_cancelled = self.total_cancelled.saturating_add(1);
    }
}

impl Default for AuctionConfig {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            team_wallet: Pubkey::default(),
            mcc_mint: Pubkey::default(),
            territory_nft_program: Pubkey::default(),
            min_bid_increment_percent: 0,
            auction_duration: 0,
            extension_duration: 0,
            extension_threshold: 0,
            next_auction_id: 0,
            total_auctions: 0,
            total_completed: 0,
            total_cancelled: 0,
            total_volume: 0,
            is_paused: false,
            is_initialized: false,
            created_at: 0,
            updated_at: 0,
            _reserved: [0u8; 64],
        }
    }
}
