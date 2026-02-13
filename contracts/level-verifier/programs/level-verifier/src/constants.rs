use anchor_lang::prelude::*;

pub const VERIFIER_CONFIG_SEED: &[u8] = b"verifier_config";

pub const USER_PROFILE_SEED: &[u8] = b"user_profile";

pub const MINING_RECORD_SEED: &[u8] = b"mining_record";

pub const LEVEL_RECRUIT: u8 = 1;

pub const LEVEL_PROSPECT: u8 = 2;

pub const LEVEL_MINER: u8 = 3;

pub const LEVEL_COMMANDER: u8 = 4;

pub const LEVEL_PIONEER: u8 = 5;

pub const LEVEL_WARDEN: u8 = 6;

pub const LEVEL_ADMIRAL: u8 = 7;

pub const MINER_REQUIRED_MINING_DAYS: u8 = 21;

pub const MINING_PERIOD_DAYS: u8 = 30;

pub const COMMANDER_REQUIRED_STATIONS: u8 = 1;

pub const PIONEER_REQUIRED_STATIONS: u8 = 10;

pub const WARDEN_REQUIRED_MATRICES: u8 = 10;

pub const ADMIRAL_REQUIRED_SECTORS: u8 = 10;

pub const TERRITORY_TYPE_STATION: u8 = 0;

pub const TERRITORY_TYPE_MATRIX: u8 = 1;

pub const TERRITORY_TYPE_SECTOR: u8 = 2;

pub const TERRITORY_TYPE_SYSTEM: u8 = 3;

pub const MAX_FIREBASE_UID_LENGTH: usize = 128;

pub const SECONDS_PER_DAY: i64 = 86400;
