// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, CloseAccount, TransferChecked};

use crate::constants::*;
use crate::error::McdError;
use crate::state::{McdConfig, StationMcdVault};

#[derive(Accounts)]
#[instruction(station_id: u64)]
pub struct CloseOldStationVaultAta<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [MCD_CONFIG_SEED],
        bump = mcd_config.bump,
        constraint = mcd_config.authority == authority.key() @ McdError::Unauthorized,
    )]
    pub mcd_config: Account<'info, McdConfig>,

    #[account(
        seeds = [STATION_VAULT_SEED, &station_id.to_le_bytes()],
        bump = station_vault.bump,
    )]
    pub station_vault: Account<'info, StationMcdVault>,

    #[account(
        mut,
        constraint = old_token_account.owner == station_vault.key() @ McdError::InvalidOwner,
    )]
    pub old_token_account: InterfaceAccount<'info, TokenAccount>,

    pub old_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = destination_token_account.mint == old_mint.key() @ McdError::InvalidMint,
    )]
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<CloseOldStationVaultAta>, station_id: u64) -> Result<()> {
    let old_ata = ctx.accounts.old_token_account.key();
    let authority = ctx.accounts.authority.key();
    let vault_bump = ctx.accounts.station_vault.bump;
    let remaining_balance = ctx.accounts.old_token_account.amount;
    let decimals = ctx.accounts.old_mint.decimals;

    msg!("Closing old Station Vault ATA");
    msg!("Station ID: {}", station_id);
    msg!("Old ATA: {}", old_ata);
    msg!("Remaining balance: {}", remaining_balance);
    msg!("Rent receiver: {}", authority);

    let station_id_bytes = station_id.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[&[
        STATION_VAULT_SEED,
        station_id_bytes.as_ref(),
        &[vault_bump],
    ]];

    if remaining_balance > 0 {
        msg!("Transferring remaining balance to destination...");

        let transfer_accounts = TransferChecked {
            from: ctx.accounts.old_token_account.to_account_info(),
            mint: ctx.accounts.old_mint.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: ctx.accounts.station_vault.to_account_info(),
        };

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        token_2022::transfer_checked(transfer_ctx, remaining_balance, decimals)?;
        msg!("Balance transferred: {}", remaining_balance);
    }

    let cpi_accounts = CloseAccount {
        account: ctx.accounts.old_token_account.to_account_info(),
        destination: ctx.accounts.authority.to_account_info(),
        authority: ctx.accounts.station_vault.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token_2022::close_account(cpi_ctx)?;

    msg!("Old ATA closed successfully, rent recovered");

    Ok(())
}
