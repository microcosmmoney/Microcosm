use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::constants::*;
use crate::state::LendingPool;

#[derive(Accounts)]
pub struct InitializeLpMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub lending_pool: Account<'info, LendingPool>,

    pub asset_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        mint::decimals = asset_mint.decimals,
        mint::authority = lending_pool,
        seeds = [LP_MINT_SEED, lending_pool.key().as_ref()],
        bump
    )]
    pub lp_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeLpMint>) -> Result<()> {
    let pool = &mut ctx.accounts.lending_pool;

    pool.lp_mint = ctx.accounts.lp_mint.key();

    msg!("LP Mint initialized for lending pool");
    msg!("LP Mint address: {}", pool.lp_mint);

    Ok(())
}
