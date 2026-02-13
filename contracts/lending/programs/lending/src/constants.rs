pub const LENDING_POOL_SEED: &[u8] = b"mcc_lending_pool";

pub const POOL_VAULT_SEED: &[u8] = b"mcc_pool_vault";

pub const LP_MINT_SEED: &[u8] = b"mcc_lp_mint";

pub const LOAN_SEED: &[u8] = b"mcc_loan";

pub const NFT_ESCROW_SEED: &[u8] = b"nft_escrow";

pub const NFT_PRICE_ORACLE_SEED: &[u8] = b"nft_price_oracle";

pub const MAX_LTV_BPS: u64 = 5000;

pub const MAX_INTEREST_RATE_BPS: u64 = 10000;

pub const PROTOCOL_FEE_BPS: u64 = 1000;

pub const LOAN_DURATION_3_DAYS: i64 = 3 * 24 * 60 * 60;

pub const LOAN_DURATION_7_DAYS: i64 = 7 * 24 * 60 * 60;

pub const LOAN_DURATION_30_DAYS: i64 = 30 * 24 * 60 * 60;

pub const DURATION_TYPE_3_DAYS: u8 = 0;
pub const DURATION_TYPE_7_DAYS: u8 = 1;
pub const DURATION_TYPE_30_DAYS: u8 = 2;

pub const BPS_DENOMINATOR: u64 = 10000;

pub const DEFAULT_BASE_RATE: u64 = 1000;

pub const DEFAULT_OPTIMAL_UTILIZATION: u64 = 8000;

pub const DEFAULT_SLOPE1: u64 = 1000;

pub const DEFAULT_SLOPE2: u64 = 8000;

pub const SECONDS_PER_YEAR: u64 = 31536000;

pub const DEFAULT_STATION_VALUE_MCC: u64 = 1_000_000_000_000;

pub const DEFAULT_MATRIX_VALUE_MCC: u64 = 15_000_000_000_000;

pub const DEFAULT_SECTOR_VALUE_MCC: u64 = 200_000_000_000_000;

pub const DEFAULT_SYSTEM_VALUE_MCC: u64 = 2_500_000_000_000_000;

pub const TERRITORY_TYPE_STATION: u8 = 0;
pub const TERRITORY_TYPE_MATRIX: u8 = 1;
pub const TERRITORY_TYPE_SECTOR: u8 = 2;
pub const TERRITORY_TYPE_SYSTEM: u8 = 3;

pub const TERRITORY_COLLECTION: &str = "BvV8jWSkLfw7GQthdrsqF6DZm91aVDbcEbpDWYNP88ST";

pub const TERRITORY_PREFIX_STATION: &str = "Microcosm Station";
pub const TERRITORY_PREFIX_MATRIX: &str = "Microcosm Matrix";
pub const TERRITORY_PREFIX_SECTOR: &str = "Microcosm Sector";
pub const TERRITORY_PREFIX_SYSTEM: &str = "Microcosm System";

pub const MIN_LOAN_AMOUNT: u64 = 100_000_000_000;

pub const MAX_LOAN_AMOUNT: u64 = 10_000_000_000_000_000;

pub const MAX_LOAN_DURATION: i64 = 365 * 24 * 60 * 60;

pub const MIN_DEPOSIT_AMOUNT: u64 = 10_000_000_000;

pub const MAX_POOL_NAME_LEN: usize = 32;
