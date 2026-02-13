use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::constants::REINCARNATION_POOL_SEED;
use crate::error::ReincarnationError;
use crate::state::ReincarnationPool;

pub fn handler(ctx: Context<MigratePool>) -> Result<()> {
    let pool_info = ctx.accounts.reincarnation_pool.to_account_info();
    let current_size = pool_info.data_len();
    let new_size = ReincarnationPool::LEN;

    msg!("Migrating ReincarnationPool account");
    msg!("Current size: {} bytes", current_size);
    msg!("Target size: {} bytes", new_size);

    if current_size >= new_size {
        msg!("Account already at correct size, no migration needed");
        return Ok(());
    }

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);
    let current_balance = pool_info.lamports();
    let lamports_diff = new_minimum_balance.saturating_sub(current_balance);

    msg!("Additional lamports needed: {}", lamports_diff);

    if lamports_diff > 0 {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: pool_info.clone(),
            },
        );
        system_program::transfer(cpi_context, lamports_diff)?;
    }

    pool_info.realloc(new_size, false)?;

    msg!("Account extended successfully: {} -> {} bytes", current_size, new_size);

    let mut data = pool_info.try_borrow_mut_data()?;

    msg!("Migration complete");

    Ok(())
}

#[derive(Accounts)]
pub struct MigratePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [REINCARNATION_POOL_SEED],
        bump,
    )]
    pub reincarnation_pool: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
