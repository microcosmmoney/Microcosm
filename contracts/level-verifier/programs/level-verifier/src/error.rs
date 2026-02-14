// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[error_code]
pub enum LevelVerifierError {

    #[msg("Verifier config already initialized")]
    AlreadyInitialized,

    #[msg("Verifier config not initialized")]
    NotInitialized,

    #[msg("User profile already exists")]
    ProfileAlreadyExists,

    #[msg("User profile not found")]
    ProfileNotFound,

    #[msg("Wallet already bound")]
    WalletAlreadyBound,

    #[msg("Invalid Firebase UID")]
    InvalidFirebaseUid,

    #[msg("Unauthorized to perform this operation")]
    Unauthorized,

    #[msg("Not the profile owner")]
    NotProfileOwner,

    #[msg("Current level does not meet upgrade requirements")]
    LevelRequirementNotMet,

    #[msg("Already at this level")]
    AlreadyAtLevel,

    #[msg("Prerequisite level not met")]
    PrerequisiteLevelNotMet,

    #[msg("Cannot downgrade to a lower level")]
    CannotDowngrade,

    #[msg("Invalid level")]
    InvalidLevel,

    #[msg("Mining already recorded today")]
    AlreadyMinedToday,

    #[msg("Insufficient mining days")]
    InsufficientMiningDays,

    #[msg("Mining record expired")]
    MiningRecordExpired,

    #[msg("Insufficient NFT count")]
    InsufficientNftCount,

    #[msg("NFT verification failed")]
    NftVerificationFailed,

    #[msg("Invalid NFT type")]
    InvalidNftType,

    #[msg("Numeric overflow")]
    Overflow,

    #[msg("Invalid argument")]
    InvalidArgument,

    #[msg("Operations paused")]
    Paused,
}
