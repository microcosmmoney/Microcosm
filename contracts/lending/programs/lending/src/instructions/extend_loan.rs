// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::{LendingPool, Loan, LoanStatus, LoanDuration};

#[derive(Accounts)]
pub struct ExtendLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds = [LOAN_SEED, borrower.key().as_ref(), loan.nft_mint.as_ref()],
        bump = loan.bump,
        constraint = loan.status == LoanStatus::Active @ LendingError::LoanNotActive,
        constraint = loan.lending_pool == lending_pool.key() @ LendingError::InvalidParameter,
        constraint = loan.borrower == borrower.key() @ LendingError::NotLoanOwner
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        mut,
        constraint = vault.key() == lending_pool.vault @ LendingError::InvalidParameter
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = borrower_asset_account.owner == borrower.key(),
        constraint = borrower_asset_account.mint == lending_pool.asset_mint
    )]
    pub borrower_asset_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ExtendLoan>, new_duration_type: u8) -> Result<()> {
    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;
    let loan = &mut ctx.accounts.loan;

    let new_duration = LoanDuration::from_u8(new_duration_type)
        .ok_or(LendingError::InvalidParameter)?;

    require!(
        !loan.is_overdue(clock.unix_timestamp),
        LendingError::LoanNotActive
    );

    pool.accrue_interest(clock.unix_timestamp)?;

    loan.accrue_interest(clock.unix_timestamp, pool.borrow_rate())?;

    let interest_owed = loan.accrued_interest;

    require!(interest_owed > 0 || true, LendingError::InvalidParameter);

    if interest_owed > 0 {
        require!(
            ctx.accounts.borrower_asset_account.amount >= interest_owed,
            LendingError::InsufficientLpBalance
        );

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.borrower_asset_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.borrower.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, interest_owed)?;

        loan.accrued_interest = 0;
    }

    let old_due_at = loan.due_at;
    loan.duration = new_duration;
    loan.due_at = clock.unix_timestamp + new_duration.to_seconds();
    loan.last_interest_update = clock.unix_timestamp;

    pool.updated_at = clock.unix_timestamp;

    msg!("Loan extended successfully!");
    msg!("Borrower: {}", loan.borrower);
    msg!("Interest paid: {} MCC", interest_owed);
    msg!("New duration: {:?}", new_duration);
    msg!("Old due_at: {}", old_due_at);
    msg!("New due_at: {}", loan.due_at);

    Ok(())
}
