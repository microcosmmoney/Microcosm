use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::McdError;

#[derive(Accounts)]
pub struct FixConfigBump<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [MCD_CONFIG_SEED],
        bump,
        owner = crate::ID,
    )]
    pub mcd_config: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<FixConfigBump>) -> Result<()> {
    let mcd_config = &ctx.accounts.mcd_config;
    let bump = ctx.bumps.mcd_config;

    msg!("Fix McdConfig bump");
    msg!("Correct bump value: {}", bump);

    let data = mcd_config.try_borrow_data()?;
    let authority_bytes = &data[8..40];
    let stored_authority = Pubkey::try_from(authority_bytes).unwrap();
    require!(stored_authority == ctx.accounts.authority.key(), McdError::Unauthorized);
    drop(data);

    let mut data = mcd_config.try_borrow_mut_data()?;
    let current_bump = data[208];
    msg!("Current bump at offset 208: {}", current_bump);

    data[208] = bump;
    msg!("Fixed bump to: {}", bump);

    Ok(())
}
