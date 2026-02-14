// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum StablecoinType {
    USDC = 0,
    USDT = 1,
}

impl Default for StablecoinType {
    fn default() -> Self {
        StablecoinType::USDC
    }
}

pub const REINCARNATION_POOL_SEED: &[u8] = b"reincarnation_pool_v2";

pub const USDC_VAULT_SEED: &[u8] = b"usdc_vault";

pub const USDT_VAULT_SEED: &[u8] = b"usdt_vault";

pub const MCC_BURN_VAULT_SEED: &[u8] = b"mcc_burn_vault";

pub const BUYBACK_RECORD_SEED: &[u8] = b"buyback_record";

pub const BUYBACK_PREMIUM_BPS: u64 = 500;

pub const BPS_DENOMINATOR: u64 = 10000;

pub const MIN_BUYBACK_AMOUNT: u64 = 10_000_000;

pub const MAX_BUYBACK_AMOUNT: u64 = 100_000_000_000_000;

pub const DAILY_LIMIT_USDC: u64 = 1_000_000_000_000;

pub const MCC_DECIMALS: u8 = 9;

pub const MCD_DECIMALS: u8 = 9;

pub const USDC_TO_MCD_FACTOR: u64 = 1000;

pub const USDC_DECIMALS: u8 = 6;

pub const DEFAULT_BASE_PRICE: u64 = 10_000_000;

pub const MAX_BASE_PRICE: u64 = 1_000_000_000;

pub const MIN_BASE_PRICE: u64 = 10_000;

pub const PRICE_VALIDITY_SECONDS: i64 = 300;

pub const MCC_MINT: &str = "MCCpDtigJLYnfGe1fW5xrSA8AXo6AeAj8ECE7wVqP5e";

pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";

pub const USDT_DECIMALS: u8 = 6;

pub const MAX_POOL_NAME_LEN: usize = 32;

pub const MCC_GENESIS_AUTHORITY: Pubkey = pubkey!("MCWe3gEYqTjRCShEkhqbMXhcR4xP2dz29gurSpLbk4A");

pub const MCD_GENESIS_POOL: Pubkey = pubkey!("DfLGbEV6FDSsG5BaFeCDLMrr9LKMupDQvENvp9943QiC");

pub const MINING_USER_PCT: u64 = 50;

pub const MINING_TEAM_PCT: u64 = 10;

pub const MINING_MAGISTRATE_PCT: u64 = 10;

pub const MINING_STATION_MAGISTRATE_PCT: u64 = 4;
pub const MINING_MATRIX_MAGISTRATE_PCT: u64 = 3;
pub const MINING_SECTOR_MAGISTRATE_PCT: u64 = 2;
pub const MINING_SYSTEM_MAGISTRATE_PCT: u64 = 1;

pub const MINING_DEVELOPER_MCD_PCT: u64 = 10;

pub const MINING_STATION_MCD_PCT_WITH_DEVELOPER: u64 = 20;

pub const MINING_STATION_MCD_PCT_NO_DEVELOPER: u64 = 30;

pub const MIN_CYCLE_INTERVAL_DAYS: i64 = 25;

pub const SECONDS_PER_DAY: i64 = 86400;
