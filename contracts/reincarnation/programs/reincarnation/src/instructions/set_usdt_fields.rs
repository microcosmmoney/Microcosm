use anchor_lang::prelude::*;

use crate::constants::REINCARNATION_POOL_SEED;

pub fn handler(ctx: Context<SetUsdtFields>, usdt_mint: Pubkey, usdt_vault: Pubkey) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;

    msg!("Setting USDT fields");
    msg!("usdt_mint: {}", usdt_mint);
    msg!("usdt_vault: {}", usdt_vault);

    let mut data = pool.try_borrow_mut_data()?;

    let usdt_mint_offset = 8 + 4 + 32 + 32 * 3 + 32 * 3;
    data[usdt_mint_offset..usdt_mint_offset + 32].copy_from_slice(&usdt_mint.to_bytes());

    let usdt_vault_offset = usdt_mint_offset + 32;
    data[usdt_vault_offset..usdt_vault_offset + 32].copy_from_slice(&usdt_vault.to_bytes());

    msg!("USDT fields set successfully");

    Ok(())
}

#[derive(Accounts)]
pub struct SetUsdtFields<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,
}
