use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenInterface, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, StationMcdVault};
use crate::McdDistribution;

#[derive(Accounts)]
#[instruction(station_id: u64)]
pub struct DistributeDailyMcd<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MCD_CONFIG_SEED],
        bump = mcd_config.bump,
        constraint = mcd_config.authority == authority.key() @ McdError::Unauthorized,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    #[account(
        mut,
        seeds = [STATION_VAULT_SEED, &station_id.to_le_bytes()],
        bump = station_vault.bump,
        constraint = station_vault.station_id == station_id @ McdError::InvalidStationId,
    )]
    pub station_vault: Account<'info, StationMcdVault>,

    #[account(
        mut,
        constraint = station_token_account.key() == station_vault.token_account @ McdError::StationVaultNotFound,
    )]
    pub station_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

}

pub fn handler<'info>(
    ctx: Context<'_, '_, 'info, 'info, DistributeDailyMcd<'info>>,
    station_id: u64,
    distributions: Vec<McdDistribution>,
) -> Result<()> {
    require!(
        distributions.len() <= MAX_DISTRIBUTIONS_PER_TX,
        McdError::TooManyDistributions
    );

    let clock = Clock::get()?;
    let today = clock.unix_timestamp / 86400 * 86400;

    require!(
        ctx.accounts.station_vault.last_distribution_date < today,
        McdError::AlreadyDistributed
    );

    let vault_balance = ctx.accounts.station_vault.balance;
    let vault_bump = ctx.accounts.station_vault.bump;

    msg!("Distribute Daily MCD");
    msg!("Station ID: {}", station_id);
    msg!("Vault balance: {} MCD", vault_balance as f64 / 1_000_000.0);

    let distribution_amount = vault_balance
        .checked_mul(VAULT_DAILY_DISTRIBUTION_RATE)
        .ok_or(McdError::MathOverflow)?
        .checked_div(VAULT_DAILY_DISTRIBUTION_DIVISOR)
        .ok_or(McdError::MathOverflow)?;

    msg!("Distribution amount (1%): {} MCD", distribution_amount as f64 / 1_000_000.0);
    msg!("Recipients: {}", distributions.len());

    let total_to_distribute: u64 = distributions.iter()
        .map(|d| d.amount)
        .sum();

    require!(
        total_to_distribute <= distribution_amount,
        McdError::InvalidAmount
    );

    require!(
        ctx.remaining_accounts.len() == distributions.len(),
        McdError::InvalidAmount
    );

    let station_id_bytes = station_id.to_le_bytes();
    let vault_seeds: &[&[u8]] = &[
        STATION_VAULT_SEED,
        &station_id_bytes,
        &[vault_bump],
    ];
    let signer_seeds = &[vault_seeds];

    for (i, distribution) in distributions.iter().enumerate() {
        let user_token_account = &ctx.remaining_accounts[i];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.station_token_account.to_account_info(),
                to: user_token_account.to_account_info(),
                authority: ctx.accounts.station_vault.to_account_info(),
            },
            signer_seeds,
        );
        token_interface::transfer(transfer_ctx, distribution.amount)?;

        msg!("Distributed {} MCD to UID {}", distribution.amount as f64 / 1_000_000.0, distribution.uid);
    }

    let vault = &mut ctx.accounts.station_vault;
    vault.balance = vault.balance
        .checked_sub(total_to_distribute)
        .ok_or(McdError::MathOverflow)?;
    vault.total_distributed = vault.total_distributed
        .checked_add(total_to_distribute)
        .ok_or(McdError::MathOverflow)?;
    vault.last_distribution_date = today;

    msg!("Distribution completed");
    msg!("Vault new balance: {} MCD", vault.balance as f64 / 1_000_000.0);

    Ok(())
}
