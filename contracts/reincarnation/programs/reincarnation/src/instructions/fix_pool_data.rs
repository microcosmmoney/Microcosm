use anchor_lang::prelude::*;

use crate::constants::REINCARNATION_POOL_SEED;

pub fn handler(
    ctx: Context<FixPoolData>,
    usdt_mint: Pubkey,
    usdt_vault: Pubkey,
    base_price: u64,
    premium_bps: u64,
    daily_limit: u64,
) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;
    let clock = Clock::get()?;

    msg!("Fixing ReincarnationPool data layout");
    msg!("usdt_mint: {}", usdt_mint);
    msg!("usdt_vault: {}", usdt_vault);
    msg!("base_price: {} (6 decimals)", base_price);
    msg!("premium_bps: {}", premium_bps);
    msg!("daily_limit: {} (6 decimals)", daily_limit);

    let mut data = pool.try_borrow_mut_data()?;

    let usdt_mint_offset = 258;
    let usdt_vault_offset = 290;
    let base_price_offset = 322;
    let premium_bps_offset = 330;
    let price_updated_at_offset = 338;
    let paused_offset = 346;
    let daily_limit_offset = 347;
    let daily_used_offset = 355;
    let last_reset_day_offset = 363;

    data[usdt_mint_offset..usdt_mint_offset + 32].copy_from_slice(&usdt_mint.to_bytes());
    msg!("usdt_mint written at offset {}", usdt_mint_offset);

    data[usdt_vault_offset..usdt_vault_offset + 32].copy_from_slice(&usdt_vault.to_bytes());
    msg!("usdt_vault written at offset {}", usdt_vault_offset);

    data[base_price_offset..base_price_offset + 8].copy_from_slice(&base_price.to_le_bytes());
    msg!("base_price written at offset {}", base_price_offset);

    data[premium_bps_offset..premium_bps_offset + 8].copy_from_slice(&premium_bps.to_le_bytes());
    msg!("premium_bps written at offset {}", premium_bps_offset);

    let timestamp = clock.unix_timestamp;
    data[price_updated_at_offset..price_updated_at_offset + 8].copy_from_slice(&timestamp.to_le_bytes());
    msg!("price_updated_at written at offset {} = {}", price_updated_at_offset, timestamp);

    data[paused_offset] = 0;
    msg!("paused written at offset {} = false", paused_offset);

    data[daily_limit_offset..daily_limit_offset + 8].copy_from_slice(&daily_limit.to_le_bytes());
    msg!("daily_limit written at offset {}", daily_limit_offset);

    data[daily_used_offset..daily_used_offset + 8].copy_from_slice(&0u64.to_le_bytes());
    msg!("daily_used written at offset {} = 0", daily_used_offset);

    let current_day = timestamp / 86400;
    data[last_reset_day_offset..last_reset_day_offset + 8].copy_from_slice(&current_day.to_le_bytes());
    msg!("last_reset_day written at offset {} = {}", last_reset_day_offset, current_day);

    msg!("Data layout fix complete");

    Ok(())
}

#[derive(Accounts)]
pub struct FixPoolData<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,
}
