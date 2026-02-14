// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use crate::constants::MAX_POOL_NAME_LEN;

#[account]
pub struct ReincarnationPool {

    pub name: String,

    pub authority: Pubkey,

    pub mcc_mint: Pubkey,

    pub usdc_mint: Pubkey,

    pub mcd_mint: Pubkey,

    pub usdc_vault: Pubkey,

    pub mcc_vault: Pubkey,

    pub mcd_vault: Pubkey,

    pub usdt_mint: Pubkey,

    pub usdt_vault: Pubkey,

    pub base_price: u64,

    pub premium_bps: u64,

    pub price_updated_at: i64,

    pub paused: bool,

    pub daily_limit: u64,

    pub daily_used: u64,

    pub last_reset_day: i64,

    pub total_mcc_bought: u64,

    pub total_usd_paid: u64,

    pub total_buyback_count: u64,

    pub total_mining_usd_received: u64,

    pub total_mining_mcc_minted: u64,

    pub total_mining_mcd_minted: u64,

    pub total_mining_count: u64,

    pub last_cycle_timestamp: i64,

    pub total_mcc_cycled: u64,

    pub total_mcd_cycled: u64,

    pub total_cycle_count: u64,

    pub created_at: i64,

    pub bump: u8,

    pub _reserved: [u8; 32],
}

impl ReincarnationPool {
    pub const LEN: usize = 8 + 4 + MAX_POOL_NAME_LEN + 32 * 10 + 8 * 17 + 1 + 8 * 4 + 1 + 32;

    pub fn calculate_buyback_price(&self) -> u64 {
        let premium_amount = self.base_price
            .checked_mul(self.premium_bps)
            .and_then(|v| v.checked_div(10000))
            .unwrap_or(0);

        self.base_price.checked_add(premium_amount).unwrap_or(self.base_price)
    }

    pub fn calculate_usdc_amount(&self, mcc_amount: u64) -> Option<u64> {
        let buyback_price = self.calculate_buyback_price();

        let product = (mcc_amount as u128)
            .checked_mul(buyback_price as u128)?;

        let usdc_amount = product.checked_div(1_000_000_000)?;

        if usdc_amount > u64::MAX as u128 {
            return None;
        }

        Some(usdc_amount as u64)
    }

    pub fn check_and_reset_daily_limit(&mut self, current_timestamp: i64) {
        let current_day = current_timestamp / 86400;

        if current_day > self.last_reset_day {
            self.daily_used = 0;
            self.last_reset_day = current_day;
        }
    }

    pub fn can_execute_buyback(&self, usdc_amount: u64) -> bool {
        if self.paused {
            return false;
        }

        let new_daily_used = self.daily_used.saturating_add(usdc_amount);
        new_daily_used <= self.daily_limit
    }
}
