// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionError {

    #[msg("Auction config already initialized")]
    AlreadyInitialized,

    #[msg("Auction config not initialized")]
    NotInitialized,

    #[msg("Invalid bid increment percent")]
    InvalidBidIncrementPercent,

    #[msg("Invalid auction duration")]
    InvalidAuctionDuration,

    #[msg("Unauthorized to perform this operation")]
    Unauthorized,

    #[msg("Not the auction creator")]
    NotAuctionCreator,

    #[msg("Not the bid owner")]
    NotBidOwner,

    #[msg("Not the NFT owner")]
    NotNftOwner,

    #[msg("Auction not found")]
    AuctionNotFound,

    #[msg("Auction has ended")]
    AuctionEnded,

    #[msg("Auction has not ended yet")]
    AuctionNotEnded,

    #[msg("Auction has been cancelled")]
    AuctionCancelled,

    #[msg("Auction has been completed")]
    AuctionCompleted,

    #[msg("Auction is in progress and cannot be cancelled")]
    AuctionInProgress,

    #[msg("Territory is already in an active auction")]
    TerritoryAlreadyInAuction,

    #[msg("Territory is not in an auction")]
    TerritoryNotInAuction,

    #[msg("Bid is below the starting price")]
    BidBelowStartingPrice,

    #[msg("Bid is below the current highest price")]
    BidBelowCurrentHighest,

    #[msg("Bid increment is insufficient")]
    BidIncrementInsufficient,

    #[msg("Bid not found")]
    BidNotFound,

    #[msg("Already has an active bid")]
    AlreadyHasActiveBid,

    #[msg("Bid has already been cancelled")]
    BidAlreadyCancelled,

    #[msg("Cannot cancel the highest bid")]
    CannotCancelHighestBid,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Maximum number of bids reached")]
    MaxBidsReached,

    #[msg("Invalid territory type")]
    InvalidTerritoryType,

    #[msg("Territory NFT not found")]
    TerritoryNftNotFound,

    #[msg("NFT minting failed")]
    NftMintFailed,

    #[msg("NFT transfer failed")]
    NftTransferFailed,

    #[msg("MCC transfer failed")]
    MccTransferFailed,

    #[msg("Escrow account error")]
    EscrowError,

    #[msg("Refund failed")]
    RefundFailed,

    #[msg("Numeric overflow")]
    Overflow,

    #[msg("Invalid argument")]
    InvalidArgument,

    #[msg("Operations paused")]
    Paused,
}
