// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::{Loan, LoanStatus};
use crate::constants::LOAN_SEED;

pub fn create_loan_escrow(ctx: Context<CreateLoanEscrow>) -> Result<()> {
    require!(
        ctx.accounts.loan.status == LoanStatus::Pending,
        crate::error::LendingError::InvalidLoanState
    );

    msg!("NFT Escrow ATA created for Loan PDA");
    msg!("  Loan: {}", ctx.accounts.loan.key());
    msg!("  NFT Mint: {}", ctx.accounts.nft_mint.key());
    msg!("  NFT Escrow: {}", ctx.accounts.nft_escrow.key());

    Ok(())
}

#[derive(Accounts)]
pub struct CreateLoanEscrow<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        seeds = [LOAN_SEED, borrower.key().as_ref(), nft_mint.key().as_ref()],
        bump = loan.bump,
        constraint = loan.borrower == borrower.key(),
        constraint = loan.nft_mint == nft_mint.key(),
        constraint = loan.status == LoanStatus::Pending @ crate::error::LendingError::InvalidLoanState
    )]
    pub loan: Account<'info, Loan>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = borrower,
        associated_token::mint = nft_mint,
        associated_token::authority = loan
    )]
    pub nft_escrow: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
