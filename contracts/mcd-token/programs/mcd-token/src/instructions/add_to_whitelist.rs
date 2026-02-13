use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, McdWhitelist};

#[derive(Accounts)]
#[instruction(project_id: u64)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MCD_CONFIG_SEED],
        bump = mcd_config.bump,
        constraint = mcd_config.authority == authority.key() @ McdError::Unauthorized,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    #[account(
        init,
        payer = authority,
        space = McdWhitelist::LEN,
        seeds = [MCD_WHITELIST_SEED, &project_id.to_le_bytes()],
        bump,
    )]
    pub whitelist: Account<'info, McdWhitelist>,

    pub wallet_address: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AddToWhitelist>,
    project_id: u64,
    project_name: String,
) -> Result<()> {
    require!(
        project_name.len() <= 64,
        McdError::ProjectNameTooLong
    );

    let whitelist = &mut ctx.accounts.whitelist;
    let clock = Clock::get()?;

    whitelist.project_id = project_id;
    whitelist.wallet_address = ctx.accounts.wallet_address.key();

    let mut name_bytes = [0u8; 64];
    let name_bytes_slice = project_name.as_bytes();
    name_bytes[..name_bytes_slice.len()].copy_from_slice(name_bytes_slice);
    whitelist.project_name = name_bytes;

    whitelist.registered_at = clock.unix_timestamp;
    whitelist.status = McdWhitelist::STATUS_ACTIVE;
    whitelist.updated_at = clock.unix_timestamp;
    whitelist.bump = ctx.bumps.whitelist;

    msg!("Added to MCD whitelist");
    msg!("Project ID: {}", project_id);
    msg!("Wallet: {}", whitelist.wallet_address);
    msg!("Project Name: {}", project_name);

    Ok(())
}
