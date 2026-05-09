use crate::errors::DocumentError;
use crate::event::DocumentRead;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn documents_read(ctx: Context<ReadDocuments>, seq: u64) -> Result<()> {
    require!(!ctx.accounts.config.paused, DocumentError::Paused);

    require!(
        !ctx.accounts.grant_read.revoked,
        DocumentError::GrantRevoked
    );

    require_keys_eq!(
        ctx.accounts.document.owner,
        ctx.accounts.owner.key(),
        DocumentError::DocumentMismatch
    );
    require_keys_eq!(
        ctx.accounts.document.notary,
        ctx.accounts.notary.key(),
        DocumentError::DocumentMismatch
    );

    require_keys_eq!(
        ctx.accounts.owner_doc_seq.owner,
        ctx.accounts.owner.key(),
        DocumentError::BadSeq
    );

    require_keys_eq!(
        ctx.accounts.grant_read.owner,
        ctx.accounts.owner.key(),
        DocumentError::GrantMismatch
    );
    require_keys_eq!(
        ctx.accounts.grant_read.grantee,
        ctx.accounts.reader.key(),
        DocumentError::GrantMismatch
    );

    emit!(DocumentRead {
        document: ctx.accounts.document.key(),
        owner: ctx.accounts.owner.key(),
        notary: ctx.accounts.notary.key(),
        reader: ctx.accounts.reader.key(),
        seq,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(seq: u64)]
pub struct ReadDocuments<'info> {
    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    pub reader: Signer<'info>,

    #[account(
        seeds = [SEED_OWNER, owner.owner_pubkey.as_ref()],
        bump = owner.bump,
    )]
    pub owner: Account<'info, PropertyOwner>,

    #[account(
        seeds = [SEED_NOTARY, notary.authority.as_ref()],
        bump = notary.bump,
    )]
    pub notary: Account<'info, Notary>,

    #[account(
        seeds = [SEED_OWNER_DOC_SEQ, owner.key().as_ref()],
        bump = owner_doc_seq.bump,
        constraint = owner_doc_seq.owner == owner.key() @ DocumentError::BadSeq,
    )]
    pub owner_doc_seq: Account<'info, OwnerDocumentSeq>,

    #[account(
        seeds = [SEED_GRANT, owner.key().as_ref(), reader.key().as_ref(), &[SCOPE_READ]],
        bump = grant_read.bump,
        constraint = grant_read.owner == owner.key()             @ DocumentError::GrantMismatch,
        constraint = grant_read.grantee == reader.key()          @ DocumentError::GrantMismatch,
        constraint = !grant_read.revoked                         @ DocumentError::GrantRevoked,
    )]
    pub grant_read: Account<'info, Grant>,

    #[account(
        seeds = [SEED_DOCUMENT, owner.key().as_ref(), &seq.to_le_bytes()],
        bump = document.bump
    )]
    pub document: Account<'info, PropertyDocument>,

    pub system_program: Program<'info, System>,
}
