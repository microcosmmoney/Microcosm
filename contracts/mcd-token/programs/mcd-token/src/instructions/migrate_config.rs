use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::constants::*;
use crate::error::McdError;
use crate::state::McdConfig;

#[account]
pub struct McdConfigV1 {
    pub authority: Pubkey,
    pub mcd_mint: Pubkey,
    pub genesis_pool: Pubkey,
    pub recycle_pool: Pubkey,
    pub total_minted: u64,
    pub total_recycled: u64,
    pub total_vault_minted: u64,
    pub total_consumed: u64,
    pub total_usdc_deposited: u64,
    pub total_usdc_withdrawn: u64,
    pub last_update_timestamp: i64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct MigrateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MCD_CONFIG_SEED],
        bump,
        owner = crate::ID,
    )]
    pub mcd_config: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MigrateConfig>) -> Result<()> {
    let mcd_config = &ctx.accounts.mcd_config;

    let current_len = mcd_config.data_len();
    let new_len = McdConfig::LEN;

    msg!("Migrate McdConfig");
    msg!("Current size: {} bytes", current_len);
    msg!("New size: {} bytes", new_len);

    if current_len >= new_len {
        msg!("Account already migrated or larger");
        return Ok(());
    }

    let data = mcd_config.try_borrow_data()?;

    let authority = Pubkey::try_from(&data[8..40]).unwrap();
    let mcd_mint = Pubkey::try_from(&data[40..72]).unwrap();
    let genesis_pool = Pubkey::try_from(&data[72..104]).unwrap();
    let recycle_pool = Pubkey::try_from(&data[104..136]).unwrap();
    let total_minted = u64::from_le_bytes(data[136..144].try_into().unwrap());
    let total_recycled = u64::from_le_bytes(data[144..152].try_into().unwrap());
    let total_vault_minted = u64::from_le_bytes(data[152..160].try_into().unwrap());
    let total_consumed = u64::from_le_bytes(data[160..168].try_into().unwrap());
    let total_usdc_deposited = u64::from_le_bytes(data[168..176].try_into().unwrap());
    let total_usdc_withdrawn = u64::from_le_bytes(data[176..184].try_into().unwrap());
    let last_update_timestamp = i64::from_le_bytes(data[184..192].try_into().unwrap());
    let bump = data[192];

    require!(authority == ctx.accounts.authority.key(), McdError::Unauthorized);

    drop(data);

    msg!("Old authority: {}", authority);
    msg!("Old bump: {}", bump);

    let additional_rent = Rent::get()?.minimum_balance(new_len) - mcd_config.lamports();

    if additional_rent > 0 {
        msg!("Transferring {} lamports for rent", additional_rent);
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.authority.to_account_info(),
                    to: mcd_config.to_account_info(),
                },
            ),
            additional_rent,
        )?;
    }

    mcd_config.realloc(new_len, false)?;

    let mut data = mcd_config.try_borrow_mut_data()?;

    data[8..40].copy_from_slice(&authority.to_bytes());
    data[40..72].copy_from_slice(&mcd_mint.to_bytes());
    data[72..104].copy_from_slice(&genesis_pool.to_bytes());
    data[104..136].copy_from_slice(&recycle_pool.to_bytes());
    data[136..144].copy_from_slice(&total_minted.to_le_bytes());
    data[144..152].copy_from_slice(&total_recycled.to_le_bytes());
    data[152..160].copy_from_slice(&total_vault_minted.to_le_bytes());
    data[160..168].copy_from_slice(&total_consumed.to_le_bytes());
    data[168..176].copy_from_slice(&total_usdc_deposited.to_le_bytes());
    data[176..184].copy_from_slice(&total_usdc_withdrawn.to_le_bytes());
    data[184..192].copy_from_slice(&0u64.to_le_bytes());
    data[192..200].copy_from_slice(&0u64.to_le_bytes());
    data[200..208].copy_from_slice(&last_update_timestamp.to_le_bytes());
    data[208] = bump;

    msg!("McdConfig migrated successfully");
    msg!("New size: {} bytes", new_len);

    Ok(())
}
