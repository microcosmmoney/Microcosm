use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {
    #[msg("Pool is not active")]
    PoolNotActive,

    #[msg("Pool name is too long")]
    PoolNameTooLong,

    #[msg("Insufficient pool liquidity")]
    InsufficientLiquidity,

    #[msg("Pool utilization is too high for withdrawal")]
    UtilizationTooHigh,

    #[msg("Cannot update - pool has deposits")]
    PoolHasDeposits,

    #[msg("Cannot update - pool has active loans")]
    PoolHasLoans,

    #[msg("Deposit amount is below minimum")]
    DepositBelowMinimum,

    #[msg("Withdrawal amount exceeds LP balance")]
    InsufficientLpBalance,

    #[msg("Insufficient shares to withdraw")]
    InsufficientShares,

    #[msg("Loan amount is below minimum")]
    LoanBelowMinimum,

    #[msg("Loan amount exceeds maximum")]
    LoanExceedsMaximum,

    #[msg("Loan amount exceeds maximum LTV")]
    ExceedsMaxLtv,

    #[msg("Loan already exists for this NFT")]
    LoanAlreadyExists,

    #[msg("Loan not found")]
    LoanNotFound,

    #[msg("Loan is not active")]
    LoanNotActive,

    #[msg("Invalid loan state")]
    InvalidLoanState,

    #[msg("Loan duration exceeds maximum")]
    LoanDurationExceeded,

    #[msg("Repayment amount is zero")]
    ZeroRepayment,

    #[msg("Repayment exceeds outstanding debt")]
    RepaymentExceedsDebt,

    #[msg("Loan is not fully repaid")]
    LoanNotFullyRepaid,

    #[msg("Loan is not liquidatable - requires 3 consecutive missed payments")]
    NotLiquidatable,

    #[msg("Missed payments below liquidation threshold (3)")]
    MissedPaymentsBelowThreshold,

    #[msg("Liquidation amount exceeds maximum")]
    LiquidationExceedsMax,

    #[msg("Invalid NFT type")]
    InvalidNftType,

    #[msg("NFT is not from Territory collection")]
    InvalidNftCollection,

    #[msg("NFT already used as collateral")]
    NftAlreadyCollateralized,

    #[msg("NFT not owned by borrower")]
    NftNotOwned,

    #[msg("NFT value not set")]
    NftValueNotSet,

    #[msg("Not pool authority")]
    NotPoolAuthority,

    #[msg("Not loan owner")]
    NotLoanOwner,

    #[msg("Not price oracle authority")]
    NotOracleAuthority,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Math underflow")]
    MathUnderflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("Invalid parameter")]
    InvalidParameter,

    #[msg("Operation is paused")]
    OperationPaused,

    #[msg("Account already initialized")]
    AlreadyInitialized,
}
