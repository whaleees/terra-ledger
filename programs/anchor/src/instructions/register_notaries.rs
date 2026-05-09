use crate::errors::NotaryError;
use crate::event::NotaryRegistered;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn notaries_register(
    ctx: Context<RegisterNotaries>,
    name: String,
    kms_ref: String,
) -> Result<()> {
    let name_trim = name.trim();
    require!(!name_trim.is_empty(), NotaryError::EmptyName);
    require!(name_trim.len() <= MAX_NAME_LEN, NotaryError::NameTooLong);

    let kms_trim = kms_ref.trim();
    require!(!kms_trim.is_empty(), NotaryError::EmptyKmsRef);
    require!(
        kms_trim.len() <= MAX_KMS_REF_LEN,
        NotaryError::KmsRefTooLong
    );

    let now = Clock::get()?.unix_timestamp;
    let notary = &mut ctx.accounts.notary;

    notary.authority = ctx.accounts.notary_authority.key();
    notary.name = name_trim.to_string();
    notary.kms_ref = kms_trim.to_string();
    notary.registered_by = ctx.accounts.registrar.key();
    notary.created_at = now;
    notary.bump = ctx.bumps.notary;

    emit!(NotaryRegistered {
        notary: notary.key(),
        notary_authority: notary.authority,
        name: notary.name.clone(),
        kms_ref: notary.kms_ref.clone(),
        registered_by: notary.registered_by,
        created_at: now,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterNotaries<'info> {
    #[account(mut)]
    pub registrar: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        constraint = config.authority == registrar.key() @ NotaryError::UnauthorizedRegistrar,
        constraint = !config.paused @ NotaryError::Paused
    )]
    pub config: Account<'info, Config>,

    pub notary_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = registrar,
        space = 8 + Notary::INIT_SPACE,
        seeds = [SEED_NOTARY, notary_authority.key().as_ref()],
        bump
    )]
    pub notary: Account<'info, Notary>,

    pub system_program: Program<'info, System>,
}
