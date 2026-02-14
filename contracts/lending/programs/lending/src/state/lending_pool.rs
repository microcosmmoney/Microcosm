// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::LendingError;

#[account]
pub struct LendingPool {
    pub name: String,

    pub authority: Pubkey,

    pub asset_mint: Pubkey,

    pub lp_mint: Pubkey,

    pub vault: Pubkey,

    pub total_deposits: u64,

    pub total_borrowed: u64,

    pub total_lp_supply: u64,

    pub accrued_interest: u64,

    pub protocol_fees: u64,

    pub base_rate: u64,

    pub optimal_utilization: u64,

    pub slope1: u64,

    pub slope2: u64,

    pub is_active: bool,

    pub deposits_paused: bool,

    pub borrows_paused: bool,

    pub last_update_timestamp: i64,

    pub created_at: i64,

    pub updated_at: i64,

    pub bump: u8,
}

impl LendingPool {
    pub const LEN: usize = 8 +
        4 + MAX_POOL_NAME_LEN +
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
        1 +
        1 +
        1 +
        8 +
        8 +
        8 +
        1;

    pub fn utilization_rate(&self) -> u64 {
        if self.total_deposits == 0 {
            return 0;
        }

        (self.total_borrowed as u128 * BPS_DENOMINATOR as u128 / self.total_deposits as u128) as u64
    }

    pub fn borrow_rate(&self) -> u64 {
        let utilization = self.utilization_rate();

        if utilization <= self.optimal_utilization {
            self.base_rate + utilization * self.slope1 / self.optimal_utilization
        } else {
            let excess_utilization = utilization - self.optimal_utilization;
            let remaining_utilization = BPS_DENOMINATOR - self.optimal_utilization;

            self.base_rate + self.slope1 + excess_utilization * self.slope2 / remaining_utilization
        }
    }

    pub fn supply_rate(&self) -> u64 {
        let borrow_rate = self.borrow_rate();
        let utilization = self.utilization_rate();

        let gross_rate = borrow_rate * utilization / BPS_DENOMINATOR;
        gross_rate * (BPS_DENOMINATOR - PROTOCOL_FEE_BPS) / BPS_DENOMINATOR
    }

    pub fn available_liquidity(&self) -> u64 {
        let total_value = self.total_deposits
            .saturating_add(self.accrued_interest)
            .saturating_sub(self.protocol_fees);
        total_value.saturating_sub(self.total_borrowed)
    }

    pub fn accrue_interest(&mut self, current_timestamp: i64) -> Result<()> {
        if current_timestamp <= self.last_update_timestamp {
            return Ok(());
        }

        let time_elapsed = (current_timestamp - self.last_update_timestamp) as u64;
        let borrow_rate = self.borrow_rate();

        let interest = self.total_borrowed
            .checked_mul(borrow_rate)
            .ok_or(LendingError::MathOverflow)?
            .checked_mul(time_elapsed)
            .ok_or(LendingError::MathOverflow)?
            / BPS_DENOMINATOR
            / SECONDS_PER_YEAR;

        let protocol_fee = interest * PROTOCOL_FEE_BPS / BPS_DENOMINATOR;

        self.accrued_interest = self.accrued_interest
            .checked_add(interest)
            .ok_or(LendingError::MathOverflow)?;

        self.protocol_fees = self.protocol_fees
            .checked_add(protocol_fee)
            .ok_or(LendingError::MathOverflow)?;

        self.last_update_timestamp = current_timestamp;
        self.updated_at = current_timestamp;

        Ok(())
    }

    pub fn lp_exchange_rate(&self) -> u64 {
        if self.total_lp_supply == 0 {
            return 1_000_000;
        }

        let total_value = self.total_deposits + self.accrued_interest - self.protocol_fees;
        (total_value as u128 * 1_000_000 / self.total_lp_supply as u128) as u64
    }

    pub fn deposit_to_lp(&self, deposit_amount: u64) -> u64 {
        if self.total_lp_supply == 0 {
            return deposit_amount;
        }

        let exchange_rate = self.lp_exchange_rate();
        (deposit_amount as u128 * 1_000_000 / exchange_rate as u128) as u64
    }

    pub fn lp_to_asset(&self, lp_amount: u64) -> u64 {
        let exchange_rate = self.lp_exchange_rate();
        (lp_amount as u128 * exchange_rate as u128 / 1_000_000) as u64
    }
}
