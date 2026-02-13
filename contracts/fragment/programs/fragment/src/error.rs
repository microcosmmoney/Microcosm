use anchor_lang::prelude::*;

#[error_code]
pub enum FragmentError {
    #[msg("Fragment count is below minimum")]
    FragmentCountBelowMinimum,

    #[msg("Fragment count exceeds maximum")]
    FragmentCountExceedsMaximum,

    #[msg("Fragment name is too long")]
    FragmentNameTooLong,

    #[msg("Fragment symbol is too long")]
    FragmentSymbolTooLong,

    #[msg("NFT already fragmented")]
    NftAlreadyFragmented,

    #[msg("NFT not fragmented")]
    NftNotFragmented,

    #[msg("Insufficient fragments for redemption")]
    InsufficientFragments,

    #[msg("Must hold all fragments to redeem")]
    NotAllFragmentsHeld,

    #[msg("NFT is locked in active buyout")]
    NftLockedInBuyout,

    #[msg("NFT not in escrow")]
    NftNotInEscrow,

    #[msg("Transfer amount is zero")]
    ZeroTransferAmount,

    #[msg("Insufficient fragment balance")]
    InsufficientBalance,

    #[msg("Cannot transfer to self")]
    CannotTransferToSelf,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid fragment mint")]
    InvalidFragmentMint,

    #[msg("Buyout already active")]
    BuyoutAlreadyActive,

    #[msg("No active buyout")]
    NoActiveBuyout,

    #[msg("Buyout has expired")]
    BuyoutExpired,

    #[msg("Buyout not expired yet")]
    BuyoutNotExpired,

    #[msg("Buyout price is too low")]
    BuyoutPriceTooLow,

    #[msg("Buyout price exceeds maximum premium")]
    BuyoutPriceExceedsMaxPremium,

    #[msg("Cannot buyout own fragments")]
    CannotBuyoutOwnFragments,

    #[msg("Insufficient funds for buyout")]
    InsufficientBuyoutFunds,

    #[msg("Buyout not complete - fragments remaining")]
    BuyoutNotComplete,

    #[msg("Not buyout initiator")]
    NotBuyoutInitiator,

    #[msg("Invalid buyout price")]
    InvalidBuyoutPrice,

    #[msg("No fragments held")]
    NoFragmentsHeld,

    #[msg("Insufficient payment")]
    InsufficientPayment,

    #[msg("No buyout in progress")]
    NoBuyoutInProgress,

    #[msg("Buyout not pending")]
    BuyoutNotPending,

    #[msg("Cannot accept own buyout")]
    CannotAcceptOwnBuyout,

    #[msg("Invalid payment mint")]
    InvalidPaymentMint,

    #[msg("Buyout already complete")]
    BuyoutAlreadyComplete,

    #[msg("Cannot cancel buyout")]
    CannotCancelBuyout,

    #[msg("Invalid initiator")]
    InvalidInitiator,

    #[msg("Buyout not cancelled or expired")]
    BuyoutNotCancelled,

    #[msg("Escrow account not empty")]
    EscrowNotEmpty,

    #[msg("Vault not active")]
    VaultNotActive,

    #[msg("Invalid vault status")]
    InvalidVaultStatus,

    #[msg("Vault is not in redeemed status")]
    VaultNotRedeemed,

    #[msg("Not original owner")]
    NotOriginalOwner,

    #[msg("Fragment mint not set")]
    FragmentMintNotSet,

    #[msg("Fragment mint mismatch")]
    FragmentMintMismatch,

    #[msg("Invalid NFT")]
    InvalidNft,

    #[msg("NFT not from Territory collection")]
    InvalidNftCollection,

    #[msg("NFT not owned by caller")]
    NftNotOwned,

    #[msg("Not authorized")]
    NotAuthorized,

    #[msg("Not authority")]
    NotAuthority,

    #[msg("Not fragment vault owner")]
    NotVaultOwner,

    #[msg("Not config authority")]
    NotConfigAuthority,

    #[msg("Invalid min fragments")]
    InvalidMinFragments,

    #[msg("Min exceeds max")]
    MinExceedsMax,

    #[msg("Invalid max fragments")]
    InvalidMaxFragments,

    #[msg("Max below min")]
    MaxBelowMin,

    #[msg("Buyout duration too short")]
    BuyoutDurationTooShort,

    #[msg("Buyout duration too long")]
    BuyoutDurationTooLong,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Math underflow")]
    MathUnderflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Invalid parameter")]
    InvalidParameter,

    #[msg("Operation is paused")]
    OperationPaused,

    #[msg("Account already initialized")]
    AlreadyInitialized,
}
