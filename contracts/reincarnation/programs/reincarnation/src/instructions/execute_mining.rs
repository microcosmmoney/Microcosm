use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use crate::constants::*;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

#[inline(never)]
fn do_transfer<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from,
        to,
        authority,
        mint,
    };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
    token_interface::transfer_checked(cpi_ctx, amount, decimals)
}

pub fn handler(
    ctx: Context<ExecuteMining>,
    mcc_amount: u64,
    usdc_amount: u64,
) -> Result<()> {
    require!(mcc_amount > 0, ReincarnationError::ZeroAmount);
    require!(usdc_amount > 0, ReincarnationError::ZeroAmount);

    let pool = &mut ctx.accounts.reincarnation_pool;

    require!(!pool.paused, ReincarnationError::PoolPaused);

    let mcc_total_pct = MINING_USER_PCT + MINING_TEAM_PCT + MINING_MAGISTRATE_PCT;
    let mcc_distribute_amount = mcc_amount
        .checked_mul(mcc_total_pct)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let user_mcc_amount = mcc_amount
        .checked_mul(MINING_USER_PCT)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let team_mcc_amount = mcc_amount
        .checked_mul(MINING_TEAM_PCT)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let station_magistrate_amount = mcc_amount
        .checked_mul(MINING_STATION_MAGISTRATE_PCT)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let matrix_magistrate_amount = mcc_amount
        .checked_mul(MINING_MATRIX_MAGISTRATE_PCT)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let sector_magistrate_amount = mcc_amount
        .checked_mul(MINING_SECTOR_MAGISTRATE_PCT)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let system_magistrate_amount = mcc_amount
        .checked_mul(MINING_SYSTEM_MAGISTRATE_PCT)
        .and_then(|v| v.checked_div(100))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    let magistrate_mcc_amount = station_magistrate_amount
        .checked_add(matrix_magistrate_amount)
        .and_then(|v| v.checked_add(sector_magistrate_amount))
        .and_then(|v| v.checked_add(system_magistrate_amount))
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    do_transfer(
        ctx.accounts.mcc_genesis_ata.to_account_info(),
        ctx.accounts.user_mcc_account.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        user_mcc_amount,
        MCC_DECIMALS,
    )?;

    do_transfer(
        ctx.accounts.mcc_genesis_ata.to_account_info(),
        ctx.accounts.team_vault.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        team_mcc_amount,
        MCC_DECIMALS,
    )?;

    do_transfer(
        ctx.accounts.mcc_genesis_ata.to_account_info(),
        ctx.accounts.station_magistrate.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        station_magistrate_amount,
        MCC_DECIMALS,
    )?;

    do_transfer(
        ctx.accounts.mcc_genesis_ata.to_account_info(),
        ctx.accounts.matrix_magistrate.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        matrix_magistrate_amount,
        MCC_DECIMALS,
    )?;

    do_transfer(
        ctx.accounts.mcc_genesis_ata.to_account_info(),
        ctx.accounts.sector_magistrate.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        sector_magistrate_amount,
        MCC_DECIMALS,
    )?;

    do_transfer(
        ctx.accounts.mcc_genesis_ata.to_account_info(),
        ctx.accounts.system_magistrate.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcc_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        system_magistrate_amount,
        MCC_DECIMALS,
    )?;

    let has_developer = ctx.accounts.developer_mcd_account.is_some();

    let (developer_mcd_amount, station_mcd_amount) = if has_developer {
        let dev_amount = usdc_amount
            .checked_mul(MINING_DEVELOPER_MCD_PCT)
            .and_then(|v| v.checked_div(100))
            .and_then(|v| v.checked_mul(USDC_TO_MCD_FACTOR))
            .ok_or(ReincarnationError::ArithmeticOverflow)?;

        let station_amount = usdc_amount
            .checked_mul(MINING_STATION_MCD_PCT_WITH_DEVELOPER)
            .and_then(|v| v.checked_div(100))
            .and_then(|v| v.checked_mul(USDC_TO_MCD_FACTOR))
            .ok_or(ReincarnationError::ArithmeticOverflow)?;

        (dev_amount, station_amount)
    } else {
        let station_amount = usdc_amount
            .checked_mul(MINING_STATION_MCD_PCT_NO_DEVELOPER)
            .and_then(|v| v.checked_div(100))
            .and_then(|v| v.checked_mul(USDC_TO_MCD_FACTOR))
            .ok_or(ReincarnationError::ArithmeticOverflow)?;

        (0u64, station_amount)
    };

    if developer_mcd_amount > 0 {
        if let Some(developer_account) = &ctx.accounts.developer_mcd_account {
            do_transfer(
                ctx.accounts.mcd_genesis_pool.to_account_info(),
                developer_account.to_account_info(),
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.mcd_mint.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                developer_mcd_amount,
                MCD_DECIMALS,
            )?;
        }
    }

    do_transfer(
        ctx.accounts.mcd_genesis_pool.to_account_info(),
        ctx.accounts.station_vault.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.mcd_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        station_mcd_amount,
        MCD_DECIMALS,
    )?;

    let mcd_amount = developer_mcd_amount
        .checked_add(station_mcd_amount)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    pool.total_mining_usd_received = pool.total_mining_usd_received
        .checked_add(usdc_amount)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    pool.total_mining_mcc_minted = pool.total_mining_mcc_minted
        .checked_add(mcc_distribute_amount)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    pool.total_mining_mcd_minted = pool.total_mining_mcd_minted
        .checked_add(mcd_amount)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    pool.total_mining_count = pool.total_mining_count
        .checked_add(1)
        .ok_or(ReincarnationError::ArithmeticOverflow)?;

    emit!(MiningExecuted {
        user: ctx.accounts.user_mcc_account.owner,
        usdc_amount,
        mcc_total: mcc_amount,
        mcc_user: user_mcc_amount,
        mcc_team: team_mcc_amount,
        mcc_station_magistrate: station_magistrate_amount,
        mcc_matrix_magistrate: matrix_magistrate_amount,
        mcc_sector_magistrate: sector_magistrate_amount,
        mcc_system_magistrate: system_magistrate_amount,
        mcd_developer: developer_mcd_amount,
        mcd_station: station_mcd_amount,
        has_developer,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteMining<'info> {

    #[account(
        mut,
        constraint = authority.key() == MCC_GENESIS_AUTHORITY @ ReincarnationError::InvalidAuthority
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump = reincarnation_pool.bump,
    )]
    pub reincarnation_pool: Box<Account<'info, ReincarnationPool>>,

    #[account(
        constraint = mcc_mint.key() == reincarnation_pool.mcc_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcc_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        constraint = mcd_mint.key() == reincarnation_pool.mcd_mint @ ReincarnationError::InvalidMccMint
    )]
    pub mcd_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        constraint = mcc_genesis_ata.mint == mcc_mint.key(),
        constraint = mcc_genesis_ata.owner == MCC_GENESIS_AUTHORITY @ ReincarnationError::InvalidGenesisAddress
    )]
    pub mcc_genesis_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = MCD_GENESIS_POOL @ ReincarnationError::InvalidGenesisAddress
    )]
    pub mcd_genesis_pool: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_mcc_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub team_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub station_magistrate: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub matrix_magistrate: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub sector_magistrate: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub system_magistrate: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub station_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub developer_mcd_account: Option<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[event]
pub struct MiningExecuted {
    pub user: Pubkey,
    pub usdc_amount: u64,
    pub mcc_total: u64,
    pub mcc_user: u64,
    pub mcc_team: u64,
    pub mcc_station_magistrate: u64,
    pub mcc_matrix_magistrate: u64,
    pub mcc_sector_magistrate: u64,
    pub mcc_system_magistrate: u64,
    pub mcd_developer: u64,
    pub mcd_station: u64,
    pub has_developer: bool,
    pub timestamp: i64,
}
