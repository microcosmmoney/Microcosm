// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

pub const AUCTION_CONFIG_SEED: &[u8] = b"auction_config";

pub const AUCTION_SEED: &[u8] = b"auction";

pub const BID_SEED: &[u8] = b"bid";

pub const MCC_ESCROW_SEED: &[u8] = b"mcc_escrow";

pub const DEFAULT_AUCTION_DURATION: i64 = 7 * 24 * 60 * 60;

pub const DEFAULT_EXTENSION_DURATION: i64 = 10 * 60;

pub const DEFAULT_EXTENSION_THRESHOLD: i64 = 5 * 60;

pub const MIN_BID_INCREMENT_PERCENT: u8 = 5;

pub const MAX_BID_INCREMENT_PERCENT: u8 = 50;

pub const MIN_STARTING_PRICE: u64 = 1_000_000_000;

pub const MAX_BIDS_PER_AUCTION: u16 = 1000;

pub const TERRITORY_TYPE_STATION: u8 = 0;

pub const TERRITORY_TYPE_MATRIX: u8 = 1;

pub const TERRITORY_TYPE_SECTOR: u8 = 2;

pub const TERRITORY_TYPE_SYSTEM: u8 = 3;

pub const FIRST_AUCTION_TEAM_SHARE: u8 = 100;

pub const SECOND_AUCTION_MAGISTRATE_SHARE: u8 = 100;
