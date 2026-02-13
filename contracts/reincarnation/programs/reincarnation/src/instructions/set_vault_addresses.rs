use anchor_lang::prelude::*;

use crate::constants::REINCARNATION_POOL_SEED;

pub fn handler(
    ctx: Context<SetVaultAddresses>,
    mcc_vault: Pubkey,
    mcd_vault: Pubkey,
) -> Result<()> {
    let pool = &ctx.accounts.reincarnation_pool;

    msg!("Updating Vault addresses");
    msg!("New mcc_vault: {}", mcc_vault);
    msg!("New mcd_vault: {}", mcd_vault);

    let mut data = pool.try_borrow_mut_data()?;

    let mcc_vault_offset = 194;
    let mcd_vault_offset = 226;

    data[mcc_vault_offset..mcc_vault_offset + 32].copy_from_slice(&mcc_vault.to_bytes());
    msg!("mcc_vault written at offset {}", mcc_vault_offset);

    data[mcd_vault_offset..mcd_vault_offset + 32].copy_from_slice(&mcd_vault.to_bytes());
    msg!("mcd_vault written at offset {}", mcd_vault_offset);

    msg!("Vault addresses update complete");

    Ok(())
}

#[derive(Accounts)]
pub struct SetVaultAddresses<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,
}
