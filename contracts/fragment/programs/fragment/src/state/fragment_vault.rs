use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::FragmentError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VaultStatus {
    Initializing,
    Active,
    Redeeming,
    Redeemed,
    BuyoutPending,
    BoughtOut,
}

impl Default for VaultStatus {
    fn default() -> Self {
        VaultStatus::Active
    }
}

#[account]
pub struct FragmentVault {
    pub original_owner: Pubkey,

    pub nft_mint: Pubkey,

    pub fragment_mint: Pubkey,

    pub fragment_name: String,

    pub fragment_symbol: String,

    pub total_fragments: u64,

    pub circulating_fragments: u64,

    pub status: VaultStatus,

    pub created_at: i64,

    pub redeemed_at: Option<i64>,

    pub bump: u8,
}

impl FragmentVault {
    pub const LEN: usize = 8 +
        32 +
        32 +
        32 +
        4 + MAX_FRAGMENT_NAME_LEN +
        4 + MAX_FRAGMENT_SYMBOL_LEN +
        8 +
        8 +
        1 +
        8 +
        1 + 8 +
        1;

    pub fn can_redeem(&self) -> bool {
        self.status == VaultStatus::Active
    }

    pub fn is_in_buyout(&self) -> bool {
        self.status == VaultStatus::BuyoutPending
    }

    pub fn mark_redeemed(&mut self, timestamp: i64) {
        self.status = VaultStatus::Redeemed;
        self.redeemed_at = Some(timestamp);
    }

    pub fn start_buyout(&mut self) {
        self.status = VaultStatus::BuyoutPending;
    }

    pub fn cancel_buyout(&mut self) {
        self.status = VaultStatus::Active;
    }

    pub fn update_circulation(&mut self, from_original: bool, to_original: bool, amount: u64) -> Result<()> {
        if from_original && !to_original {
            self.circulating_fragments = self.circulating_fragments
                .checked_add(amount)
                .ok_or(FragmentError::MathOverflow)?;
        } else if !from_original && to_original {
            self.circulating_fragments = self.circulating_fragments
                .checked_sub(amount)
                .ok_or(FragmentError::MathUnderflow)?;
        }
        Ok(())
    }
}
