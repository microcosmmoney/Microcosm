// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(ctx: Context<ExecuteMonthlyCycle>) -> Result<()> {
    let clock = Clock::get()?;
    let current_ts = clock.unix_timestamp;

    let pool_bump = ctx.accounts.reincarnation_pool.bump;
    let last_cycle_timestamp = ctx.accounts.reincarnation_pool.last_cycle_timestamp;

    let day_of_month = calculate_day_of_month(current_ts);
    require!(day_of_month == 1, ReincarnationError::NotFirstDayOfMonth);

    let time_since_last_cycle = current_ts.saturating_sub(last_cycle_timestamp);
    let min_interval = MIN_CYCLE_INTERVAL_DAYS * SECONDS_PER_DAY;
    require!(
        time_since_last_cycle > min_interval,
        ReincarnationError::CycleAlreadyExecuted
    );

    let mcc_vault_balance = ctx.accounts.mcc_vault.amount;
    let mcd_vault_balance = ctx.accounts.mcd_vault.amount;

    require!(
        mcc_vault_balance > 0 || mcd_vault_balance > 0,
        ReincarnationError::MccVaultEmpty
    );

    let signer_seeds: &[&[&[u8]]] = &[&[REINCARNATION_POOL_SEED, &[pool_bump]]];

    if mcc_vault_balance > 0 {
        let cpi_accounts_mcc = TransferChecked {
            from: ctx.accounts.mcc_vault.to_account_info(),
            to: ctx.accounts.mcc_genesis_ata.to_account_info(),
            authority: ctx.accounts.reincarnation_pool.to_account_info(),
            mint: ctx.accounts.mcc_mint.to_account_info(),
        };
        let cpi_ctx_mcc = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts_mcc,
            signer_seeds,
        );
        token_interface::transfer_checked(cpi_ctx_mcc, mcc_vault_balance, MCC_DECIMALS)?;
    }

    if mcd_vault_balance > 0 {
        let cpi_accounts_mcd = TransferChecked {
            from: ctx.accounts.mcd_vault.to_account_info(),
            to: ctx.accounts.mcd_genesis_pool.to_account_info(),
            authority: ctx.accounts.reincarnation_pool.to_account_info(),
            mint: ctx.accounts.mcd_mint.to_account_info(),
        };
        let cpi_ctx_mcd = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts_mcd,
            signer_seeds,
        );
        token_interface::transfer_checked(cpi_ctx_mcd, mcd_vault_balance, MCD_DECIMALS)?;
    }

    let pool = &mut ctx.accounts.reincarnation_pool;
    pool.last_cycle_timestamp = current_ts;
    pool.total_mcc_cycled = pool.total_mcc_cycled
        .checked_add(mcc_vault_balance)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;
    pool.total_mcd_cycled = pool.total_mcd_cycled
        .checked_add(mcd_vault_balance)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;
    pool.total_cycle_count = pool.total_cycle_count
        .checked_add(1)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    emit!(MonthlyCycleExecuted {
        authority: ctx.accounts.authority.key(),
        mcc_amount: mcc_vault_balance,
        mcd_amount: mcd_vault_balance,
        cycle_count: pool.total_cycle_count,
        timestamp: current_ts,
    });

    Ok(())
}

fn calculate_day_of_month(timestamp: i64) -> u8 {
    let days_since_epoch = timestamp / SECONDS_PER_DAY;

    let day_in_month = ((days_since_epoch % 30) + 1) as u8;

    if day_in_month <= 3 {
        1
    } else {
        day_in_month
    }
}

#[derive(Accounts)]
pub struct ExecuteMonthlyCycle<'info> {

    #[account(
        mut,
        constraint = authority.key() == reincarnation_pool.authority @ ReincarnationError::Unauthorized
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
    )]
    pub reincarnation_pool: Account<'info, ReincarnationPool>,

    #[account(
        constraint = mcc_mint.key() == reincarnation_pool.mcc_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        constraint = mcd_mint.key() == reincarnation_pool.mcd_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcd_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = mcc_vault.key() == reincarnation_pool.mcc_vault @ ReincarnationError::InvalidMccMint
    )]
    pub mcc_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mcd_vault.key() == reincarnation_pool.mcd_vault @ ReincarnationError::InvalidMccMint
    )]
    pub mcd_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = mcc_genesis_ata.owner == MCC_GENESIS_AUTHORITY @ ReincarnationError::InvalidGenesisAddress
    )]
    pub mcc_genesis_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        address = MCD_GENESIS_POOL @ ReincarnationError::InvalidGenesisAddress
    )]
    pub mcd_genesis_pool: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct MonthlyCycleExecuted {
    pub authority: Pubkey,
    pub mcc_amount: u64,
    pub mcd_amount: u64,
    pub cycle_count: u64,
    pub timestamp: i64,
}
