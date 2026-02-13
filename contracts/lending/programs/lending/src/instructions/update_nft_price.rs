use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LendingError;
use crate::state::{LendingPool, NftPriceOracle, CollateralType};

#[derive(Accounts)]
pub struct UpdateNftPrice<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [LENDING_POOL_SEED, lending_pool.name.as_bytes()],
        bump = lending_pool.bump
    )]
    pub lending_pool: Account<'info, LendingPool>,

    #[account(
        mut,
        seeds = [NFT_PRICE_ORACLE_SEED, lending_pool.key().as_ref()],
        bump = nft_price_oracle.bump,
        constraint = nft_price_oracle.authority == authority.key() @ LendingError::NotOracleAuthority
    )]
    pub nft_price_oracle: Account<'info, NftPriceOracle>,
}

pub fn handler(
    ctx: Context<UpdateNftPrice>,
    territory_type: u8,
    value_mcc: u64,
) -> Result<()> {
    let collateral_type = CollateralType::from_u8(territory_type)
        .ok_or(LendingError::InvalidNftType)?;

    require!(value_mcc > 0, LendingError::InvalidParameter);

    let clock = Clock::get()?;
    let oracle = &mut ctx.accounts.nft_price_oracle;

    let old_value = oracle.get_value(&collateral_type);
    oracle.set_value(&collateral_type, value_mcc);
    oracle.last_update = clock.unix_timestamp;

    msg!("NFT value updated");
    msg!("Type: {:?}", collateral_type);
    msg!("Old value: {} MCC", old_value);
    msg!("New value: {} MCC", value_mcc);

    Ok(())
}
