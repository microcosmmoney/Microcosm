use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::DhcError;
use crate::state::MiningConfig;

#[derive(Accounts)]
pub struct ProcessMiningPayment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_usdc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_mcc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub rebirth_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [MINING_CONFIG_SEED],
        bump = mining_config.bump,
    )]
    pub mining_config: Account<'info, MiningConfig>,

    #[account(
        mut,
        seeds = [FOUNDER_VAULT_SEED],
        bump,
    )]
    pub founder_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub team_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ProcessMiningPayment>, usdc_amount: u64) -> Result<()> {
    require!(!ctx.accounts.mining_config.is_halted, DhcError::SystemHalted);

    require!(usdc_amount > 0, DhcError::InvalidPaymentAmount);

    let config = &mut ctx.accounts.mining_config;

    let total_mcc_amount = config.calculate_mcc_amount(usdc_amount)?;

    msg!("User paid: {} USDC", usdc_amount as f64 / 1_000_000.0);
    msg!("MCC to yield: {} MCC", total_mcc_amount as f64 / 1_000_000_000.0);
    msg!("Current phase: Phase {}, Rate: {}x",
        config.current_phase,
        config.current_mining_rate as f64 / 100.0
    );

    let new_total = config.total_minted
        .checked_add(total_mcc_amount)
        .ok_or(DhcError::MathOverflow)?;

    require!(new_total <= TOTAL_SUPPLY, DhcError::MaxSupplyReached);

    let (team_amount, treasury_amount, mining_pool_amount) =
        config.calculate_distribution(total_mcc_amount)?;

    msg!("Distribution:");
    msg!("  - Team (20%): {} MCC", team_amount as f64 / 1_000_000_000.0);
    msg!("  - Treasury (20%): {} MCC", treasury_amount as f64 / 1_000_000_000.0);
    msg!("  - Mining pool (60%): {} MCC", mining_pool_amount as f64 / 1_000_000_000.0);

    let transfer_usdc_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_usdc_account.to_account_info(),
            to: ctx.accounts.rebirth_usdc_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(transfer_usdc_ctx, usdc_amount)?;

    let seeds = &[
        FOUNDER_VAULT_SEED,
        &[ctx.bumps.founder_vault],
    ];
    let signer = &[&seeds[..]];

    let transfer_team_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.founder_vault.to_account_info(),
            to: ctx.accounts.team_vault.to_account_info(),
            authority: ctx.accounts.founder_vault.to_account_info(),
        },
        signer,
    );
    token::transfer(transfer_team_ctx, team_amount)?;

    let transfer_treasury_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.founder_vault.to_account_info(),
            to: ctx.accounts.treasury_vault.to_account_info(),
            authority: ctx.accounts.founder_vault.to_account_info(),
        },
        signer,
    );
    token::transfer(transfer_treasury_ctx, treasury_amount)?;

    let transfer_mining_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.founder_vault.to_account_info(),
            to: ctx.accounts.user_mcc_account.to_account_info(),
            authority: ctx.accounts.founder_vault.to_account_info(),
        },
        signer,
    );
    token::transfer(transfer_mining_ctx, mining_pool_amount)?;

    config.total_minted = new_total;
    config.team_total = config.team_total
        .checked_add(team_amount)
        .ok_or(DhcError::MathOverflow)?;
    config.treasury_total = config.treasury_total
        .checked_add(treasury_amount)
        .ok_or(DhcError::MathOverflow)?;
    config.mining_pool_total = config.mining_pool_total
        .checked_add(mining_pool_amount)
        .ok_or(DhcError::MathOverflow)?;
    config.total_usdc_paid = config.total_usdc_paid
        .checked_add(usdc_amount)
        .ok_or(DhcError::MathOverflow)?;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    config.update_phase()?;

    msg!("Mining payment processed successfully");
    msg!("Total minted: {} MCC / {} MCC",
        config.total_minted as f64 / 1_000_000_000.0,
        TOTAL_SUPPLY as f64 / 1_000_000_000.0
    );
    msg!("Cumulative USDC: ${}", config.total_usdc_paid as f64 / 1_000_000.0);

    Ok(())
}
