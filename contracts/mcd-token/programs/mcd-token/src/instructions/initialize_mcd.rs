use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeMcd<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = MCD_DECIMALS,
        mint::authority = mcd_mint_authority,
        mint::freeze_authority = mcd_mint_authority,
        mint::token_program = token_program,
    )]
    pub mcd_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [MCD_CONFIG_SEED],
        bump,
    )]
    pub mcd_mint_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeMcd>, decimals: u8) -> Result<()> {
    msg!("Initialize MCD Token Mint");
    msg!("Decimals: {}", decimals);
    msg!("MCD Mint: {}", ctx.accounts.mcd_mint.key());
    msg!("Mint Authority (MCD Config PDA): {}", ctx.accounts.mcd_mint_authority.key());

    Ok(())
}
