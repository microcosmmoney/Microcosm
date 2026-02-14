// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

pub const FRAGMENT_CONFIG_SEED: &[u8] = b"fragment_config";

pub const FRAGMENT_VAULT_SEED: &[u8] = b"fragment_vault";

pub const FRAGMENT_MINT_SEED: &[u8] = b"fragment_mint";

pub const BUYOUT_SEED: &[u8] = b"buyout";

pub const BUYOUT_VAULT_SEED: &[u8] = b"buyout_vault";

pub const DEFAULT_FRAGMENT_COUNT: u64 = 1000;

pub const MIN_FRAGMENT_COUNT: u64 = 100;

pub const MAX_FRAGMENT_COUNT: u64 = 10000;

pub const FRAGMENT_DECIMALS: u8 = 0;

pub const DEFAULT_BUYOUT_DURATION: i64 = 7 * 24 * 60 * 60;

pub const MIN_BUYOUT_DURATION: i64 = 24 * 60 * 60;

pub const MAX_BUYOUT_DURATION: i64 = 30 * 24 * 60 * 60;

pub const MAX_BUYOUT_PREMIUM_BPS: u64 = 20000;

pub const BPS_DENOMINATOR: u64 = 10000;

pub const MAX_FRAGMENT_NAME_LEN: usize = 32;

pub const MAX_FRAGMENT_SYMBOL_LEN: usize = 10;

pub const MIN_BUYOUT_PRICE: u64 = 1_000;
