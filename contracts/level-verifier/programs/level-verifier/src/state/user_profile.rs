use anchor_lang::prelude::*;
use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserLevel {
    #[default]
    Recruit = 1,
    Prospect = 2,
    Miner = 3,
    Commander = 4,
    Pioneer = 5,
    Warden = 6,
    Admiral = 7,
}

impl UserLevel {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(UserLevel::Recruit),
            2 => Some(UserLevel::Prospect),
            3 => Some(UserLevel::Miner),
            4 => Some(UserLevel::Commander),
            5 => Some(UserLevel::Pioneer),
            6 => Some(UserLevel::Warden),
            7 => Some(UserLevel::Admiral),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            UserLevel::Recruit => 1,
            UserLevel::Prospect => 2,
            UserLevel::Miner => 3,
            UserLevel::Commander => 4,
            UserLevel::Pioneer => 5,
            UserLevel::Warden => 6,
            UserLevel::Admiral => 7,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            UserLevel::Recruit => "Recruit",
            UserLevel::Prospect => "Prospect",
            UserLevel::Miner => "Miner",
            UserLevel::Commander => "Commander",
            UserLevel::Pioneer => "Pioneer",
            UserLevel::Warden => "Warden",
            UserLevel::Admiral => "Admiral",
        }
    }
}

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub wallet: Pubkey,

    pub firebase_uid: String,

    pub level: UserLevel,

    pub wallet_bound_at: Option<i64>,

    pub first_mining_at: Option<i64>,

    pub last_mining_at: Option<i64>,

    pub mining_days_in_period: u8,

    pub mining_bitmap: u32,

    pub mining_bitmap_start_day: i64,

    pub station_nft_count: u16,

    pub matrix_nft_count: u16,

    pub sector_nft_count: u16,

    pub system_nft_count: u16,

    pub level_upgraded_at: Option<i64>,

    pub created_at: i64,

    pub updated_at: i64,

    pub bump: u8,

    pub _reserved: [u8; 32],
}

impl UserProfile {
    pub const SPACE: usize = 8 +
        32 +
        4 + MAX_FIREBASE_UID_LENGTH +
        1 +
        9 +
        9 +
        9 +
        1 +
        4 +
        8 +
        2 +
        2 +
        2 +
        2 +
        9 +
        8 +
        8 +
        1 +
        32;

    pub fn bind_wallet(&mut self, current_time: i64) {
        self.wallet_bound_at = Some(current_time);
        if self.level == UserLevel::Recruit {
            self.level = UserLevel::Prospect;
            self.level_upgraded_at = Some(current_time);
        }
    }

    pub fn record_mining(&mut self, current_time: i64) -> bool {
        let current_day = current_time / SECONDS_PER_DAY;

        if self.mining_bitmap_start_day == 0
           || current_day - self.mining_bitmap_start_day >= MINING_PERIOD_DAYS as i64
        {
            self.mining_bitmap = 0;
            self.mining_bitmap_start_day = current_day;
            self.mining_days_in_period = 0;
        }

        let day_offset = (current_day - self.mining_bitmap_start_day) as u32;
        if day_offset >= MINING_PERIOD_DAYS as u32 {
            return false;
        }

        let bit_mask = 1u32 << day_offset;

        if self.mining_bitmap & bit_mask != 0 {
            return false;
        }

        self.mining_bitmap |= bit_mask;
        self.mining_days_in_period = self.mining_bitmap.count_ones() as u8;

        if self.first_mining_at.is_none() {
            self.first_mining_at = Some(current_time);
        }
        self.last_mining_at = Some(current_time);

        true
    }

    pub fn can_upgrade_to_miner(&self) -> bool {
        self.level == UserLevel::Prospect
            && self.mining_days_in_period >= MINER_REQUIRED_MINING_DAYS
    }

    pub fn can_upgrade_to_commander(&self) -> bool {
        self.level.to_u8() >= LEVEL_MINER
            && self.station_nft_count >= COMMANDER_REQUIRED_STATIONS as u16
    }

    pub fn can_upgrade_to_pioneer(&self) -> bool {
        self.level.to_u8() >= LEVEL_COMMANDER
            && self.station_nft_count >= PIONEER_REQUIRED_STATIONS as u16
    }

    pub fn can_upgrade_to_warden(&self) -> bool {
        self.level.to_u8() >= LEVEL_PIONEER
            && self.matrix_nft_count >= WARDEN_REQUIRED_MATRICES as u16
    }

    pub fn can_upgrade_to_admiral(&self) -> bool {
        self.level.to_u8() >= LEVEL_WARDEN
            && self.sector_nft_count >= ADMIRAL_REQUIRED_SECTORS as u16
    }

    pub fn upgrade_to(&mut self, new_level: UserLevel, current_time: i64) {
        self.level = new_level;
        self.level_upgraded_at = Some(current_time);
        self.updated_at = current_time;
    }

    pub fn check_demotion(&self) -> Option<UserLevel> {
        match self.level {
            UserLevel::Admiral => {
                if self.sector_nft_count < ADMIRAL_REQUIRED_SECTORS as u16 {
                    return Some(UserLevel::Warden);
                }
            }
            UserLevel::Warden => {
                if self.matrix_nft_count < WARDEN_REQUIRED_MATRICES as u16 {
                    return Some(UserLevel::Pioneer);
                }
            }
            UserLevel::Pioneer => {
                if self.station_nft_count < PIONEER_REQUIRED_STATIONS as u16 {
                    return Some(UserLevel::Commander);
                }
            }
            UserLevel::Commander => {
                if self.station_nft_count < COMMANDER_REQUIRED_STATIONS as u16 {
                    return Some(UserLevel::Miner);
                }
            }
            _ => {}
        }
        None
    }

    pub fn update_nft_counts(
        &mut self,
        stations: u16,
        matrices: u16,
        sectors: u16,
        systems: u16,
    ) {
        self.station_nft_count = stations;
        self.matrix_nft_count = matrices;
        self.sector_nft_count = sectors;
        self.system_nft_count = systems;
    }
}
