use anchor_lang::prelude::*;

use crate::constants::REINCARNATION_POOL_SEED;

pub fn handler(ctx: Context<SetMcdFields>, mcd_mint: Pubkey, mcd_vault: Pubkey) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;

    msg!("Setting MCD fields");
    msg!("mcd_mint: {}", mcd_mint);
    msg!("mcd_vault: {}", mcd_vault);

    let mut data = pool.try_borrow_mut_data()?;

    let mcd_mint_offset = 8 + 4 + 32 + 32 + 32 + 32;
    data[mcd_mint_offset..mcd_mint_offset + 32].copy_from_slice(&mcd_mint.to_bytes());

    let mcd_vault_offset = mcd_mint_offset + 32 + 32 + 32;
    data[mcd_vault_offset..mcd_vault_offset + 32].copy_from_slice(&mcd_vault.to_bytes());

    msg!("MCD fields set successfully");

    Ok(())
}

#[derive(Accounts)]
pub struct SetMcdFields<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,
}
