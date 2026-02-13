use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::{LendingPool, Loan, LoanStatus, NftPriceOracle};

#[derive(Accounts)]
pub struct LiquidateLoan<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub borrower: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        seeds = [NFT_PRICE_ORACLE_SEED, lending_pool.key().as_ref()],
        bump = nft_price_oracle.bump
    )]
    pub nft_price_oracle: Account<'info, NftPriceOracle>,

    #[account(
        mut,
        seeds = [LOAN_SEED, borrower.key().as_ref(), loan.nft_mint.as_ref()],
        bump = loan.bump,
        constraint = loan.status == LoanStatus::Active @ LendingError::LoanNotActive,
        constraint = loan.lending_pool == lending_pool.key() @ LendingError::InvalidParameter
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        mut,
        constraint = vault.key() == lending_pool.vault @ LendingError::InvalidParameter
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = liquidator_asset_account.owner == liquidator.key(),
        constraint = liquidator_asset_account.mint == lending_pool.asset_mint
    )]
    pub liquidator_asset_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = nft_escrow.mint == loan.nft_mint,
        constraint = nft_escrow.amount == 1
    )]
    pub nft_escrow: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = liquidator_nft_account.owner == liquidator.key(),
        constraint = liquidator_nft_account.mint == loan.nft_mint
    )]
    pub liquidator_nft_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<LiquidateLoan>) -> Result<()> {
    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;
    let loan = &mut ctx.accounts.loan;
    let oracle = &ctx.accounts.nft_price_oracle;

    pool.accrue_interest(clock.unix_timestamp)?;

    loan.accrue_interest(clock.unix_timestamp, pool.borrow_rate())?;

    loan.collateral_value = oracle.get_value(&loan.collateral_type);

    require!(loan.is_liquidatable(clock.unix_timestamp), LendingError::NotLiquidatable);

    let total_debt = loan.total_debt();

    let liquidation_cost = total_debt;

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.liquidator_asset_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.liquidator.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, liquidation_cost)?;

    let loan_seeds = &[
        LOAN_SEED,
        ctx.accounts.borrower.key.as_ref(),
        loan.nft_mint.as_ref(),
        &[loan.bump],
    ];
    let signer_seeds = &[&loan_seeds[..]];

    let transfer_nft_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.nft_escrow.to_account_info(),
            to: ctx.accounts.liquidator_nft_account.to_account_info(),
            authority: loan.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_nft_ctx, 1)?;

    let close_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft_escrow.to_account_info(),
            destination: ctx.accounts.liquidator.to_account_info(),
            authority: loan.to_account_info(),
        },
        signer_seeds,
    );
    token::close_account(close_ctx)?;

    loan.mark_liquidated(clock.unix_timestamp);

    pool.total_borrowed = pool.total_borrowed
        .checked_sub(loan.principal)
        .ok_or(LendingError::MathUnderflow)?;
    pool.updated_at = clock.unix_timestamp;

    msg!("Loan liquidated (overdue)!");
    msg!("Liquidator: {}", ctx.accounts.liquidator.key());
    msg!("Debt repaid: {} MCC", total_debt);
    msg!("Due at: {} (overdue)", loan.due_at);
    msg!("NFT transferred to liquidator");

    Ok(())
}
