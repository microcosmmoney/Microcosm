use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::{LendingPool, Loan, LoanStatus};

#[derive(Accounts)]
pub struct CreateLoanExecute<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump,
        constraint = lending_pool.is_active @ LendingError::PoolNotActive,
        constraint = !lending_pool.borrows_paused @ LendingError::OperationPaused
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds = [LOAN_SEED, borrower.key().as_ref(), nft_mint.key().as_ref()],
        bump = loan.bump,
        constraint = loan.status == LoanStatus::Pending @ LendingError::LoanNotActive,
        constraint = loan.borrower == borrower.key() @ LendingError::NotLoanOwner
    )]
    pub loan: Account<'info, Loan>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = borrower_nft_account.owner == borrower.key(),
        constraint = borrower_nft_account.mint == nft_mint.key(),
        constraint = borrower_nft_account.amount == 1 @ LendingError::NftNotOwned
    )]
    pub borrower_nft_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = nft_escrow.owner == loan.key(),
        constraint = nft_escrow.mint == nft_mint.key()
    )]
    pub nft_escrow: Account<'info, TokenAccount>,

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

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CreateLoanExecute>) -> Result<()> {
    let clock = Clock::get()?;
    let loan_amount = ctx.accounts.loan.principal;

    let pool = &mut ctx.accounts.lending_pool;

    require!(
        loan_amount <= pool.available_liquidity(),
        LendingError::InsufficientLiquidity
    );

    let transfer_nft_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.borrower_nft_account.to_account_info(),
            to: ctx.accounts.nft_escrow.to_account_info(),
            authority: ctx.accounts.borrower.to_account_info(),
        },
    );
    token::transfer(transfer_nft_ctx, 1)?;

    msg!("NFT transferred to escrow");

    let pool_name = pool.name.clone();
    let pool_seeds = &[
        LENDING_POOL_SEED,
        pool_name.as_bytes(),
        &[pool.bump],
    ];
    let signer_seeds = &[&pool_seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.borrower_asset_account.to_account_info(),
            authority: pool.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, loan_amount)?;

    msg!("MCC transferred to borrower: {}", loan_amount);

    let loan = &mut ctx.accounts.loan;
    loan.status = LoanStatus::Active;

    pool.total_borrowed = pool.total_borrowed
        .checked_add(loan_amount)
        .ok_or(LendingError::MathOverflow)?;
    pool.updated_at = clock.unix_timestamp;

    msg!("Loan activated successfully (Fixed-term mode)");
    msg!("Borrower: {}", loan.borrower);
    msg!("Principal: {} MCC", loan_amount);
    msg!("Collateral value: {} MCC", loan.collateral_value);
    msg!("LTV: {} BPS (max 50%)", loan.current_ltv());
    msg!("Duration: {:?}", loan.duration);
    msg!("Due at: {} (unix timestamp)", loan.due_at);

    Ok(())
}
