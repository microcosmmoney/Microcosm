// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum BidStatus {
    #[default]
    Active,
    Withdrawn,
    Won,
}

#[account]
#[derive(Default)]
pub struct Bid {
    pub auction_id: u64,

    pub bidder: Pubkey,

    pub amount: u64,

    pub status: BidStatus,

    pub placed_at: i64,

    pub withdrawn_at: Option<i64>,

    pub is_refunded: bool,

    pub bump: u8,

    pub _reserved: [u8; 16],
}

impl Bid {
    pub const SPACE: usize = 8 +
        8 +
        32 +
        8 +
        1 +
        8 +
        9 +
        1 +
        1 +
        16;

    pub fn can_cancel(&self) -> bool {
        self.status == BidStatus::Active && !self.is_refunded
    }

    pub fn withdraw(&mut self, current_time: i64) {
        self.status = BidStatus::Withdrawn;
        self.withdrawn_at = Some(current_time);
    }

    pub fn mark_won(&mut self) {
        self.status = BidStatus::Won;
    }

    pub fn mark_refunded(&mut self) {
        self.is_refunded = true;
    }
}
