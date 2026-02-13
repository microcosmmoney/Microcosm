use anchor_lang::prelude::*;

#[error_code]
pub enum DhcError {
    #[msg("MCC total supply has reached the maximum limit")]
    MaxSupplyReached,

    #[msg("Invalid mining phase")]
    InvalidMiningPhase,

    #[msg("Invalid price or price is zero")]
    InvalidPrice,

    #[msg("Invalid USDC payment amount")]
    InvalidPaymentAmount,

    #[msg("Overflow when calculating MCC amount")]
    DhcAmountOverflow,

    #[msg("Distribution calculation error")]
    DistributionCalculationError,

    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Mining config not initialized")]
    MiningConfigNotInitialized,

    #[msg("Token already initialized")]
    AlreadyInitialized,

    #[msg("Math operation overflow")]
    MathOverflow,

    #[msg("Invalid mining rate")]
    InvalidMiningRate,

    #[msg("Insufficient minable supply remaining")]
    InsufficientMinableSupply,

    #[msg("Invalid withdrawal amount")]
    InvalidAmount,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Slippage exceeded limit")]
    SlippageExceeded,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("System halted due to excessive oracle price deviation")]
    SystemHalted,

    #[msg("Invalid halt status value")]
    InvalidHaltStatus,

    #[msg("Stack overflow, please use two-step creation")]
    StackOverflow,

    #[msg("Pool already initialized")]
    PoolAlreadyInitialized,
}
