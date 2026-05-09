// INSTRUCTION FOR CREATING / UPDATING PROPERTY OWNER DATA

use crate::errors::OwnerError;
use crate::event::OwnerUpserted;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn owner_upsert(ctx: Context<UpsertOwner>, owner_ref: String) -> Result<()> {
    require!(
        owner_ref.len() <= MAX_OWNER_REF_LEN,
        OwnerError::OwnerRefTooLong
    );

    let now = Clock::get()?.unix_timestamp;

    let owner = &mut ctx.accounts.owner;
    let new_owner = owner.created_at == 0;

    if new_owner {
        owner.owner_pubkey = ctx.accounts.authority.key();
        owner.owner_ref = owner_ref;
        owner.created_at = now;
        owner.bump = ctx.bumps.owner;

        let seq = &mut ctx.accounts.owner_doc_seq;
        seq.owner = owner.key();
        seq.value = 0;
        seq.bump = ctx.bumps.owner_doc_seq;
    } else {
        require_keys_eq!(
            owner.owner_pubkey,
            ctx.accounts.authority.key(),
            OwnerError::Unauthorized
        );
        owner.owner_ref = owner_ref;
    }

    emit!(OwnerUpserted {
        owner: owner.key(),
        owner_pubkey: owner.owner_pubkey,
        created: new_owner,
        created_at: owner.created_at,
        updated_at: now,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpsertOwner<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + PropertyOwner::INIT_SPACE,
        seeds = [SEED_OWNER, authority.key().as_ref()],
        bump
    )]
    pub owner: Account<'info, PropertyOwner>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 32 + 8 + 1,
        seeds = [SEED_OWNER_DOC_SEQ, owner.key().as_ref()],
        bump
    )]
    pub owner_doc_seq: Account<'info, OwnerDocumentSeq>,

    pub system_program: Program<'info, System>,
}
