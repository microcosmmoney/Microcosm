// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::{LendingPool, NftPriceOracle};

#[derive(Accounts)]
pub struct InitializeNftOracle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        init,
        payer = authority,
        space = NftPriceOracle::LEN,
        seeds = [NFT_PRICE_ORACLE_SEED, lending_pool.key().as_ref()],
        bump
    )]
    pub nft_price_oracle: Account<'info, NftPriceOracle>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeNftOracle>) -> Result<()> {
    let clock = Clock::get()?;
    let oracle = &mut ctx.accounts.nft_price_oracle;

    oracle.authority = ctx.accounts.authority.key();
    oracle.initialize_default_values();
    oracle.last_update = clock.unix_timestamp;
    oracle.created_at = clock.unix_timestamp;
    oracle.bump = ctx.bumps.nft_price_oracle;

    msg!("NFT price oracle initialized");
    msg!("Station value: {} MCC", oracle.station_value / 1_000_000_000);
    msg!("Matrix value: {} MCC", oracle.matrix_value / 1_000_000_000);

    Ok(())
}
