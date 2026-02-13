use anchor_lang::prelude::*;

#[error_code]
pub enum McdError {
    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Insufficient MCD balance")]
    InsufficientBalance,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Station vault not found")]
    StationVaultNotFound,

    #[msg("User MCD account not found")]
    UserAccountNotFound,

    #[msg("Too many distributions in single transaction")]
    TooManyDistributions,

    #[msg("Invalid station ID")]
    InvalidStationId,

    #[msg("Invalid user ID")]
    InvalidUserId,

    #[msg("Invalid project ID")]
    InvalidProjectId,

    #[msg("Distribution already processed for this date")]
    AlreadyDistributed,

    #[msg("Account already initialized")]
    AlreadyInitialized,

    #[msg("Invalid MCD mint")]
    InvalidMint,

    #[msg("Slippage exceeded")]
    SlippageExceeded,

    #[msg("Insufficient liquidity in USDC vault")]
    InsufficientLiquidity,

    #[msg("Project name too long (max 64 bytes)")]
    ProjectNameTooLong,

    #[msg("Project not in whitelist")]
    NotInWhitelist,

    #[msg("Project suspended")]
    ProjectSuspended,

    #[msg("Invalid status")]
    InvalidStatus,

    #[msg("Account not empty, cannot close")]
    AccountNotEmpty,

    #[msg("Token account has non-zero balance")]
    NonZeroBalance,

    #[msg("Invalid token account owner")]
    InvalidOwner,
}
