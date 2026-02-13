use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::LendingError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LoanStatus {
    Pending,
    Active,
    Repaid,
    Liquidated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LoanDuration {
    ThreeDays = 0,
    SevenDays = 1,
    ThirtyDays = 2,
}

impl LoanDuration {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(LoanDuration::ThreeDays),
            1 => Some(LoanDuration::SevenDays),
            2 => Some(LoanDuration::ThirtyDays),
            _ => None,
        }
    }

    pub fn to_seconds(&self) -> i64 {
        match self {
            LoanDuration::ThreeDays => LOAN_DURATION_3_DAYS,
            LoanDuration::SevenDays => LOAN_DURATION_7_DAYS,
            LoanDuration::ThirtyDays => LOAN_DURATION_30_DAYS,
        }
    }
}

impl Default for LoanStatus {
    fn default() -> Self {
        LoanStatus::Pending
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollateralType {
    Station = 0,
    Matrix = 1,
    Sector = 2,
    System = 3,
}

impl CollateralType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(CollateralType::Station),
            1 => Some(CollateralType::Matrix),
            2 => Some(CollateralType::Sector),
            3 => Some(CollateralType::System),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CollateralType::Station => 0,
            CollateralType::Matrix => 1,
            CollateralType::Sector => 2,
            CollateralType::System => 3,
        }
    }

    pub fn default_value_mcc(&self) -> u64 {
        match self {
            CollateralType::Station => DEFAULT_STATION_VALUE_MCC,
            CollateralType::Matrix => DEFAULT_MATRIX_VALUE_MCC,
            CollateralType::Sector => DEFAULT_SECTOR_VALUE_MCC,
            CollateralType::System => DEFAULT_SYSTEM_VALUE_MCC,
        }
    }
}

#[account]
pub struct Loan {
    pub borrower: Pubkey,

    pub lending_pool: Pubkey,

    pub nft_mint: Pubkey,

    pub collateral_type: CollateralType,

    pub collateral_value: u64,

    pub principal: u64,

    pub accrued_interest: u64,

    pub borrow_rate_at_origination: u64,

    pub status: LoanStatus,

    pub duration: LoanDuration,

    pub created_at: i64,

    pub due_at: i64,

    pub last_interest_update: i64,

    pub repaid_at: Option<i64>,

    pub liquidated_at: Option<i64>,

    pub bump: u8,
}

impl Loan {
    pub const LEN: usize = 8 +
        32 +
        32 +
        32 +
        1 +
        8 +
        8 +
        8 +
        8 +
        1 +
        1 +
        8 +
        8 +
        8 +
        1 + 8 +
        1 + 8 +
        1;

    pub fn total_debt(&self) -> u64 {
        self.principal.saturating_add(self.accrued_interest)
    }

    pub fn is_liquidatable(&self, current_timestamp: i64) -> bool {
        self.status == LoanStatus::Active &&
        current_timestamp > self.due_at
    }

    pub fn is_overdue(&self, current_timestamp: i64) -> bool {
        current_timestamp > self.due_at
    }

    pub fn time_remaining(&self, current_timestamp: i64) -> i64 {
        if current_timestamp >= self.due_at {
            0
        } else {
            self.due_at - current_timestamp
        }
    }

    pub fn current_ltv(&self) -> u64 {
        if self.collateral_value == 0 {
            return 0;
        }

        (self.total_debt() as u128 * BPS_DENOMINATOR as u128 / self.collateral_value as u128) as u64
    }

    pub fn max_borrow_amount(&self) -> u64 {
        self.collateral_value * MAX_LTV_BPS / BPS_DENOMINATOR
    }

    pub fn accrue_interest(&mut self, current_timestamp: i64, current_borrow_rate: u64) -> Result<()> {
        if current_timestamp <= self.last_interest_update {
            return Ok(());
        }

        let time_elapsed = (current_timestamp - self.last_interest_update) as u64;

        let rate = current_borrow_rate.min(MAX_INTEREST_RATE_BPS);

        let interest = self.principal
            .checked_mul(rate)
            .ok_or(LendingError::MathOverflow)?
            .checked_mul(time_elapsed)
            .ok_or(LendingError::MathOverflow)?
            / BPS_DENOMINATOR
            / SECONDS_PER_YEAR;

        self.accrued_interest = self.accrued_interest
            .checked_add(interest)
            .ok_or(LendingError::MathOverflow)?;

        self.last_interest_update = current_timestamp;

        Ok(())
    }

    pub fn repay(&mut self, amount: u64) -> Result<u64> {
        let total_debt = self.total_debt();

        if amount > total_debt {
            return Err(LendingError::RepaymentExceedsDebt.into());
        }

        if amount <= self.accrued_interest {
            self.accrued_interest = self.accrued_interest
                .checked_sub(amount)
                .ok_or(LendingError::MathUnderflow)?;
        } else {
            let remaining = amount - self.accrued_interest;
            self.accrued_interest = 0;
            self.principal = self.principal
                .checked_sub(remaining)
                .ok_or(LendingError::MathUnderflow)?;
        }

        Ok(self.total_debt())
    }

    pub fn mark_repaid(&mut self, timestamp: i64) {
        self.status = LoanStatus::Repaid;
        self.repaid_at = Some(timestamp);
    }

    pub fn mark_liquidated(&mut self, timestamp: i64) {
        self.status = LoanStatus::Liquidated;
        self.liquidated_at = Some(timestamp);
    }
}
