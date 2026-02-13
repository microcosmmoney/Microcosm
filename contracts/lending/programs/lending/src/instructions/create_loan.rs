use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use mpl_token_metadata::accounts::Metadata as MetadataAccount;

use crate::constants::*;
use crate::error::LendingError;
use crate::state::{LendingPool, Loan, LoanStatus, LoanDuration, NftPriceOracle, CollateralType};

#[derive(Accounts)]
pub struct CreateLoan<'info> {
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
        seeds = [NFT_PRICE_ORACLE_SEED, lending_pool.key().as_ref()],
        bump = nft_price_oracle.bump
    )]
    pub nft_price_oracle: Account<'info, NftPriceOracle>,

    #[account(
        init,
        payer = borrower,
        space = Loan::LEN,
        seeds = [LOAN_SEED, borrower.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub loan: Account<'info, Loan>,

    pub nft_mint: UncheckedAccount<'info>,

    #[account(
        seeds = [
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = mpl_token_metadata::ID,
        bump
    )]
    pub nft_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = borrower_nft_account.owner == borrower.key(),
        constraint = borrower_nft_account.mint == nft_mint.key(),
        constraint = borrower_nft_account.amount == 1 @ LendingError::NftNotOwned
    )]
    pub borrower_nft_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = borrower,
        associated_token::mint = nft_mint,
        associated_token::authority = loan
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
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateLoan>, loan_amount: u64, duration_type: u8) -> Result<()> {
    require!(
        loan_amount >= MIN_LOAN_AMOUNT,
        LendingError::LoanBelowMinimum
    );
    require!(
        loan_amount <= MAX_LOAN_AMOUNT,
        LendingError::LoanExceedsMaximum
    );

    let duration = LoanDuration::from_u8(duration_type)
        .ok_or(LendingError::InvalidParameter)?;

    let clock = Clock::get()?;

    let lending_pool_key = ctx.accounts.lending_pool.key();

    let pool = &mut ctx.accounts.lending_pool;
    let oracle = &ctx.accounts.nft_price_oracle;

    pool.accrue_interest(clock.unix_timestamp)?;

    require!(
        loan_amount <= pool.available_liquidity(),
        LendingError::InsufficientLiquidity
    );

    let metadata_account_info = &ctx.accounts.nft_metadata.to_account_info();
    let metadata = MetadataAccount::safe_deserialize(&metadata_account_info.data.borrow())
        .map_err(|_| LendingError::InvalidNftType)?;

    if let Some(collection) = &metadata.collection {
        let expected_collection = Pubkey::try_from(TERRITORY_COLLECTION)
            .map_err(|_| LendingError::InvalidParameter)?;

        require!(
            collection.key == expected_collection,
            LendingError::InvalidNftCollection
        );

    } else {
        return Err(LendingError::InvalidNftCollection.into());
    }

    let nft_name = metadata.name.trim_end_matches('\0');
    let collateral_type = if nft_name.starts_with(TERRITORY_PREFIX_SYSTEM) {
        CollateralType::System
    } else if nft_name.starts_with(TERRITORY_PREFIX_SECTOR) {
        CollateralType::Sector
    } else if nft_name.starts_with(TERRITORY_PREFIX_MATRIX) {
        CollateralType::Matrix
    } else if nft_name.starts_with(TERRITORY_PREFIX_STATION) {
        CollateralType::Station
    } else {
        return Err(LendingError::InvalidNftType.into());
    };

    msg!("NFT Metadata verified: {} -> {:?}", nft_name, collateral_type);

    let collateral_value = oracle.get_value(&collateral_type);
    require!(collateral_value > 0, LendingError::NftValueNotSet);

    let max_borrow = collateral_value * MAX_LTV_BPS / BPS_DENOMINATOR;
    require!(loan_amount <= max_borrow, LendingError::ExceedsMaxLtv);

    let transfer_nft_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.borrower_nft_account.to_account_info(),
            to: ctx.accounts.nft_escrow.to_account_info(),
            authority: ctx.accounts.borrower.to_account_info(),
        },
    );
    token::transfer(transfer_nft_ctx, 1)?;

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

    let loan = &mut ctx.accounts.loan;
    loan.borrower = ctx.accounts.borrower.key();
    loan.lending_pool = lending_pool_key;
    loan.nft_mint = ctx.accounts.nft_mint.key();
    loan.collateral_type = collateral_type;
    loan.collateral_value = collateral_value;
    loan.principal = loan_amount;
    loan.accrued_interest = 0;
    loan.borrow_rate_at_origination = pool.borrow_rate();
    loan.status = LoanStatus::Active;
    loan.duration = duration;
    loan.created_at = clock.unix_timestamp;
    loan.due_at = clock.unix_timestamp + duration.to_seconds();
    loan.last_interest_update = clock.unix_timestamp;
    loan.repaid_at = None;
    loan.liquidated_at = None;
    loan.bump = ctx.bumps.loan;

    pool.total_borrowed = pool.total_borrowed
        .checked_add(loan_amount)
        .ok_or(LendingError::MathOverflow)?;
    pool.updated_at = clock.unix_timestamp;

    msg!("MCC Loan created successfully (Fixed-term mode)");
    msg!("Borrower: {}", loan.borrower);
    msg!("Principal: {} MCC", loan_amount);
    msg!("Collateral value: {} MCC", collateral_value);
    msg!("LTV: {} BPS (max 50%)", loan.current_ltv());
    msg!("Duration: {:?}", duration);
    msg!("Due at: {} (unix timestamp)", loan.due_at);

    Ok(())
}
