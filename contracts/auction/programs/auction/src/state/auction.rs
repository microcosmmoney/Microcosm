// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum AuctionType {
    #[default]
    First,
    Secondary,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuctionStatus {
    #[default]
    Pending,
    Active,
    Completed,
    Cancelled,
}

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
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TerritoryType::Station),
            1 => Some(TerritoryType::Matrix),
            2 => Some(TerritoryType::Sector),
            3 => Some(TerritoryType::System),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            TerritoryType::Station => 0,
            TerritoryType::Matrix => 1,
            TerritoryType::Sector => 2,
            TerritoryType::System => 3,
        }
    }
}

#[account]
#[derive(Default)]
pub struct Auction {
    pub auction_id: u64,

    pub auction_type: AuctionType,

    pub status: AuctionStatus,

    pub territory_type: TerritoryType,

    pub territory_id: u64,

    pub creator: Pubkey,

    pub starting_price: u64,

    pub buy_now_price: Option<u64>,

    pub current_price: u64,

    pub highest_bidder: Option<Pubkey>,

    pub bid_count: u16,

    pub start_time: i64,

    pub end_time: i64,

    pub extension_count: u16,

    pub proceeds_recipient: Pubkey,

    pub settled_amount: u64,

    pub nft_mint: Option<Pubkey>,

    pub created_at: i64,

    pub completed_at: Option<i64>,

    pub bump: u8,

    pub _reserved: [u8; 32],
}

impl Auction {
    pub const SPACE: usize = 8 +
        8 +
        1 +
        1 +
        1 +
        8 +
        32 +
        8 +
        9 +
        8 +
        33 +
        2 +
        8 +
        8 +
        2 +
        32 +
        8 +
        33 +
        8 +
        9 +
        1 +
        32;

    pub fn is_active(&self) -> bool {
        self.status == AuctionStatus::Active
    }

    pub fn is_ended(&self, current_time: i64) -> bool {
        current_time >= self.end_time
    }

    pub fn can_bid(&self, current_time: i64) -> bool {
        self.is_active() && !self.is_ended(current_time)
    }

    pub fn min_bid_amount(&self, increment_percent: u8) -> u64 {
        if self.current_price == 0 {
            self.starting_price
        } else {
            let increment = self.current_price
                .saturating_mul(increment_percent as u64)
                .saturating_div(100);
            self.current_price.saturating_add(increment.max(1))
        }
    }

    pub fn should_extend(&self, current_time: i64, threshold: i64) -> bool {
        let remaining = self.end_time.saturating_sub(current_time);
        remaining > 0 && remaining <= threshold
    }

    pub fn extend(&mut self, extension_duration: i64) {
        self.end_time = self.end_time.saturating_add(extension_duration);
        self.extension_count = self.extension_count.saturating_add(1);
    }

    pub fn update_highest_bid(&mut self, bidder: Pubkey, amount: u64) {
        self.highest_bidder = Some(bidder);
        self.current_price = amount;
        self.bid_count = self.bid_count.saturating_add(1);
    }

    pub fn complete(&mut self, current_time: i64) {
        self.status = AuctionStatus::Completed;
        self.settled_amount = self.current_price;
        self.completed_at = Some(current_time);
    }

    pub fn cancel(&mut self) {
        self.status = AuctionStatus::Cancelled;
    }
}
