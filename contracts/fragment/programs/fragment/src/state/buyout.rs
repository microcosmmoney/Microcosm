use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::FragmentError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BuyoutStatus {
    Initializing,
    Pending,
    Completed,
    Cancelled,
    Expired,
}

impl Default for BuyoutStatus {
    fn default() -> Self {
        BuyoutStatus::Pending
    }
}

#[account]
pub struct Buyout {
    pub vault: Pubkey,

    pub initiator: Pubkey,

    pub price_per_fragment: u64,

    pub total_buyout_amount: u64,

    pub fragments_to_buy: u64,

    pub fragments_accepted: u64,

    pub payment_collected: u64,

    pub payment_mint: Pubkey,

    pub status: BuyoutStatus,

    pub initiated_at: i64,

    pub expires_at: i64,

    pub completed_at: Option<i64>,

    pub bump: u8,
}

impl Buyout {
    pub const LEN: usize = 8 +
        32 +
        32 +
        8 +
        8 +
        8 +
        8 +
        8 +
        32 +
        1 +
        8 +
        8 +
        1 + 8 +
        1;

    pub fn is_pending(&self) -> bool {
        self.status == BuyoutStatus::Pending
    }

    pub fn is_expired(&self, current_time: i64) -> bool {
        current_time > self.expires_at
    }

    pub fn can_complete(&self) -> bool {
        self.fragments_accepted >= self.fragments_to_buy
    }

    pub fn remaining_fragments(&self) -> u64 {
        self.fragments_to_buy.saturating_sub(self.fragments_accepted)
    }

    pub fn price_for_fragments(&self, amount: u64) -> u64 {
        amount
            .checked_mul(self.price_per_fragment)
            .unwrap_or(u64::MAX)
    }

    pub fn record_acquisition(&mut self, amount: u64, payment: u64) -> Result<()> {
        self.fragments_accepted = self.fragments_accepted
            .checked_add(amount)
            .ok_or(FragmentError::MathOverflow)?;
        self.payment_collected = self.payment_collected
            .checked_add(payment)
            .ok_or(FragmentError::MathOverflow)?;
        Ok(())
    }

    pub fn mark_completed(&mut self, timestamp: i64) {
        self.status = BuyoutStatus::Completed;
        self.completed_at = Some(timestamp);
    }

    pub fn mark_cancelled(&mut self) {
        self.status = BuyoutStatus::Cancelled;
    }

    pub fn mark_expired(&mut self) {
        self.status = BuyoutStatus::Expired;
    }
}
