use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::DhcError;
use crate::state::MiningConfig;

#[derive(Accounts)]
pub struct AdminMiningReward<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MINING_CONFIG_SEED],
        bump = mining_config.bump,
        constraint = mining_config.authority == authority.key() @ DhcError::Unauthorized,
    )]
    pub mining_config: Account<'info, MiningConfig>,

    #[account(
        mut,
        seeds = [FOUNDER_VAULT_SEED],
        bump,
    )]
    pub founder_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_mcc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub team_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<AdminMiningReward>,
    mcc_amount: u64,
) -> Result<()> {
    require!(!ctx.accounts.mining_config.is_halted, DhcError::SystemHalted);

    require!(mcc_amount > 0, DhcError::InvalidPaymentAmount);

    let config = &mut ctx.accounts.mining_config;

    msg!("=== Admin Mining Reward (50-10-10-30) ===");
    msg!("MCC total amount: {} MCC", mcc_amount as f64 / 1_000_000_000.0);

    let onchain_mcc = mcc_amount
        .checked_mul(70)
        .ok_or(DhcError::MathOverflow)?
        .checked_div(100)
        .ok_or(DhcError::MathOverflow)?;

    let new_total = config.total_minted
        .checked_add(onchain_mcc)
        .ok_or(DhcError::MathOverflow)?;

    require!(new_total <= TOTAL_SUPPLY, DhcError::MaxSupplyReached);

    let (team_amount, magistrate_amount, user_amount) =
        config.calculate_distribution(mcc_amount)?;

    msg!("Distribution (50-10-10):");
    msg!("  User (50%): {} MCC", user_amount as f64 / 1_000_000_000.0);
    msg!("  Team (10%): {} MCC", team_amount as f64 / 1_000_000_000.0);
    msg!("  Magistrate (10%): {} MCC", magistrate_amount as f64 / 1_000_000_000.0);
    msg!("  Station MCD (30%): {} MCD (off-chain)",
         (mcc_amount - user_amount - team_amount - magistrate_amount) as f64 / 1_000_000_000.0);

    let seeds = &[
        FOUNDER_VAULT_SEED,
        &[ctx.bumps.founder_vault],
    ];
    let signer = &[&seeds[..]];

    if team_amount > 0 {
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
    }

    if magistrate_amount > 0 {
        let transfer_magistrate_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.founder_vault.to_account_info(),
                to: ctx.accounts.treasury_vault.to_account_info(),
                authority: ctx.accounts.founder_vault.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_magistrate_ctx, magistrate_amount)?;
    }

    if user_amount > 0 {
        let transfer_user_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.founder_vault.to_account_info(),
                to: ctx.accounts.user_mcc_account.to_account_info(),
                authority: ctx.accounts.founder_vault.to_account_info(),
            },
            signer,
        );
        token::transfer(transfer_user_ctx, user_amount)?;
    }

    config.total_minted = new_total;
    config.team_total = config.team_total
        .checked_add(team_amount)
        .ok_or(DhcError::MathOverflow)?;
    config.treasury_total = config.treasury_total
        .checked_add(magistrate_amount)
        .ok_or(DhcError::MathOverflow)?;
    config.mining_pool_total = config.mining_pool_total
        .checked_add(user_amount)
        .ok_or(DhcError::MathOverflow)?;

    let clock = Clock::get()?;
    config.last_update_timestamp = clock.unix_timestamp;

    msg!("=== Mining completed ===");
    msg!("Total minted: {} / {} MCC",
        config.total_minted as f64 / 1_000_000_000.0,
        TOTAL_SUPPLY as f64 / 1_000_000_000.0
    );

    Ok(())
}
