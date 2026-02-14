// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::REINCARNATION_POOL_SEED;

pub fn handler(
    ctx: Context<SetTokenMints>,
    mcc_mint: Pubkey,
    mcd_mint: Pubkey,
) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;

    msg!("Updating Token Mints");
    msg!("New mcc_mint: {}", mcc_mint);
    msg!("New mcd_mint: {}", mcd_mint);

    let mut data = pool.try_borrow_mut_data()?;

    let mcc_mint_offset = 66;
    let mcd_mint_offset = 130;

    data[mcc_mint_offset..mcc_mint_offset + 32].copy_from_slice(&mcc_mint.to_bytes());
    msg!("mcc_mint written at offset {}", mcc_mint_offset);

    data[mcd_mint_offset..mcd_mint_offset + 32].copy_from_slice(&mcd_mint.to_bytes());
    msg!("mcd_mint written at offset {}", mcd_mint_offset);

    msg!("Token Mints update complete");

    Ok(())
}

#[derive(Accounts)]
pub struct SetTokenMints<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,
}
