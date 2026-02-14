// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[error_code]
pub enum ReincarnationError {
    #[msg("Pool is not active")]
    PoolNotActive,

    #[msg("Pool is paused")]
    PoolPaused,

    #[msg("Pool is not paused (must pause before close)")]
    PoolNotPaused,

    #[msg("Insufficient USDC in pool")]
    InsufficientPoolBalance,

    #[msg("Daily limit exceeded")]
    DailyLimitExceeded,

    #[msg("Amount is below minimum")]
    AmountBelowMinimum,

    #[msg("Amount exceeds maximum")]
    AmountExceedsMaximum,

    #[msg("Amount is zero")]
    ZeroAmount,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Price is invalid")]
    InvalidPrice,

    #[msg("Price is stale")]
    StalePrice,

    #[msg("Price exceeds maximum")]
    PriceExceedsMaximum,

    #[msg("Price is below minimum")]
    PriceBelowMinimum,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Invalid MCC mint")]
    InvalidMccMint,

    #[msg("Invalid USDC mint")]
    InvalidUsdcMint,

    #[msg("Insufficient MCC balance")]
    InsufficientMccBalance,

    #[msg("Token transfer failed")]
    TokenTransferFailed,

    #[msg("Pool name too long")]
    PoolNameTooLong,

    #[msg("Invalid pool name")]
    InvalidPoolName,

    #[msg("Already initialized")]
    AlreadyInitialized,

    #[msg("Invalid USDT mint")]
    InvalidUsdtMint,

    #[msg("Invalid stablecoin type")]
    InvalidStablecoinType,

    #[msg("Invalid genesis address")]
    InvalidGenesisAddress,

    #[msg("Station vault not initialized")]
    StationVaultNotInitialized,

    #[msg("Insufficient MCC in genesis wallet")]
    InsufficientMccInGenesis,

    #[msg("Insufficient MCD in genesis pool")]
    InsufficientMcdInGenesis,

    #[msg("Invalid developer MCD account")]
    InvalidDeveloperAccount,

    #[msg("Not the first day of month")]
    NotFirstDayOfMonth,

    #[msg("Monthly cycle already executed this month")]
    CycleAlreadyExecuted,

    #[msg("Insufficient time since last cycle")]
    InsufficientTimeSinceLastCycle,

    #[msg("MCC vault is empty")]
    MccVaultEmpty,

    #[msg("MCD vault is empty")]
    McdVaultEmpty,

    #[msg("Invalid vault authority")]
    InvalidVault,

    #[msg("Vault has non-zero balance, cannot close")]
    VaultNotEmpty,
}
