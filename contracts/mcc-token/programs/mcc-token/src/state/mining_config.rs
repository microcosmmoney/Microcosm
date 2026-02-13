use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MiningConfig {
    pub authority: Pubkey,

    pub current_price: u64,

    pub current_phase: u8,

    pub current_mining_rate: u64,

    pub total_minted: u64,

    pub team_total: u64,

    pub treasury_total: u64,

    pub mining_pool_total: u64,

    pub total_usdc_paid: u64,

    pub total_cycled: u64,

    pub last_update_timestamp: i64,

    pub created_at: i64,

    pub is_halted: bool,

    pub bump: u8,
}

impl MiningConfig {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1;

    pub fn update_phase(&mut self) -> Result<()> {
        use crate::constants::*;

        let new_phase = (self.total_minted / PHASE_THRESHOLD) as u8;

        if new_phase != self.current_phase && new_phase < TOTAL_PHASES {
            self.current_phase = new_phase;
            self.current_mining_rate = MINING_RATES[new_phase as usize];
            msg!("Mining phase updated: Phase {} -> Rate: {}", new_phase, self.current_mining_rate);
        }

        Ok(())
    }

    pub fn calculate_mcc_amount(&self, usdc_amount: u64) -> Result<u64> {
        use crate::error::DhcError;

        require!(self.current_price > 0, DhcError::InvalidPrice);

        let base_mcc = usdc_amount
            .checked_div(self.current_price)
            .ok_or(DhcError::MathOverflow)?;

        let mcc_with_rate = base_mcc
            .checked_mul(self.current_mining_rate)
            .ok_or(DhcError::DhcAmountOverflow)?
            .checked_div(100)
            .ok_or(DhcError::MathOverflow)?;

        let mcc_amount = mcc_with_rate
            .checked_mul(1_000_000_000)
            .ok_or(DhcError::DhcAmountOverflow)?;

        Ok(mcc_amount)
    }

    pub fn calculate_distribution(&self, total_mcc: u64) -> Result<(u64, u64, u64)> {
        use crate::constants::*;
        use crate::error::DhcError;

        let team_amount = total_mcc
            .checked_mul(DISTRIBUTION_TEAM_PCT as u64)
            .ok_or(DhcError::MathOverflow)?
            .checked_div(100)
            .ok_or(DhcError::MathOverflow)?;

        let magistrate_amount = total_mcc
            .checked_mul(DISTRIBUTION_MAGISTRATE_PCT as u64)
            .ok_or(DhcError::MathOverflow)?
            .checked_div(100)
            .ok_or(DhcError::MathOverflow)?;

        let user_amount = total_mcc
            .checked_mul(DISTRIBUTION_MINING_PCT as u64)
            .ok_or(DhcError::MathOverflow)?
            .checked_div(100)
            .ok_or(DhcError::MathOverflow)?;

        Ok((team_amount, magistrate_amount, user_amount))
    }
}
