use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount};

use crate::constants::*;
use crate::error::LendingError;
use crate::state::{LendingPool, Loan, LoanStatus};

#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub borrower: UncheckedAccount<'info>,

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
        constraint = payer_asset_account.owner == payer.key(),
        constraint = payer_asset_account.mint == lending_pool.asset_mint
    )]
    pub payer_asset_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = nft_escrow.mint == loan.nft_mint,
        constraint = nft_escrow.amount == 1
    )]
    pub nft_escrow: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = borrower_nft_account.owner == borrower.key(),
        constraint = borrower_nft_account.mint == loan.nft_mint
    )]
    pub borrower_nft_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RepayLoan>, amount: u64) -> Result<()> {
    require!(amount > 0, LendingError::ZeroRepayment);

    let clock = Clock::get()?;
    let pool = &mut ctx.accounts.lending_pool;
    let loan = &mut ctx.accounts.loan;

    pool.accrue_interest(clock.unix_timestamp)?;

    loan.accrue_interest(clock.unix_timestamp, pool.borrow_rate())?;

    let total_debt = loan.total_debt();
    let actual_repayment = amount.min(total_debt);

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.payer_asset_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, actual_repayment)?;

    let remaining_debt = loan.repay(actual_repayment)?;

    let principal_repaid = loan.principal.min(actual_repayment);
    pool.total_borrowed = pool.total_borrowed
        .checked_sub(principal_repaid)
        .ok_or(LendingError::MathUnderflow)?;

    msg!("MCC Repayment received: {} MCC", actual_repayment);
    msg!("Remaining debt: {} MCC", remaining_debt);

    if remaining_debt == 0 {
        loan.mark_repaid(clock.unix_timestamp);

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
                to: ctx.accounts.borrower_nft_account.to_account_info(),
                authority: loan.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_nft_ctx, 1)?;

        let close_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.nft_escrow.to_account_info(),
                destination: ctx.accounts.payer.to_account_info(),
                authority: loan.to_account_info(),
            },
            signer_seeds,
        );
        token::close_account(close_ctx)?;

        msg!("Loan fully repaid!");
        msg!("NFT returned to borrower");
    }

    pool.updated_at = clock.unix_timestamp;

    Ok(())
}
