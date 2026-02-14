// AI-generated · AI-managed · AI-maintained
use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::LevelVerifierError;
use crate::state::VerifierConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        constraint = authority.key() == verifier_config.authority @ LevelVerifierError::Unauthorized
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [VERIFIER_CONFIG_SEED],
        bump
    )]
    pub verifier_config: Account<'info, VerifierConfig>,
}

pub fn handler(
    ctx: Context<UpdateConfig>,
    territory_nft_program: Option<Pubkey>,
    mcc_token_program: Option<Pubkey>,
) -> Result<()> {
    let config = &mut ctx.accounts.verifier_config;
    let clock = Clock::get()?;

    if let Some(program) = territory_nft_program {
        msg!("Updating territory_nft_program: {} -> {}", config.territory_nft_program, program);
        config.territory_nft_program = program;
    }

    if let Some(program) = mcc_token_program {
        msg!("Updating mcc_token_program: {} -> {}", config.mcc_token_program, program);
        config.mcc_token_program = program;
    }

    config.updated_at = clock.unix_timestamp;

    msg!("VerifierConfig updated");
    msg!("Authority: {}", config.authority);
    msg!("Territory NFT Program: {}", config.territory_nft_program);
    msg!("MCC Token Program: {}", config.mcc_token_program);

    Ok(())
}
