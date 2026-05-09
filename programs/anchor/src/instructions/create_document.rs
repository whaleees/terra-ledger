use crate::errors::DocumentError;
use crate::event::DocumentCreated;
use crate::states::*;
use anchor_lang::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn document_create(
    ctx: Context<CreateDocument>,
    seq: u64,
    cid_enc: String,
    meta_mime: String,
    meta_cid: String,
    size_bytes: u64,
    blake2b_256: [u8; 32],
    edek_root: Vec<u8>,
    edek_for_owner: Vec<u8>,
    edek_for_notary: Vec<u8>,
    edek_root_algo: WrapAlgo,
    edek_owner_algo: WrapAlgo,
    edek_notary_algo: WrapAlgo,
    kms_ref: String,
    enc_version: u16,
    enc_algo: EncAlgo,
    notary_name: String,
    document_type: String,
    property_ref: String,
    counterparty_name: String,
) -> Result<()> {
    // 1) Global pause
    require!(!ctx.accounts.config.paused, DocumentError::Paused);

    // 2) Notary authority must match uploader (notary wallet)
    require_keys_eq!(
        ctx.accounts.notary.authority,
        ctx.accounts.uploader.key(),
        DocumentError::UploaderNotNotaryAuthority
    );

    // 3) Payer must be the property owner (owner funds the init/rent)
    require_keys_eq!(
        ctx.accounts.payer.key(),
        ctx.accounts.owner.owner_pubkey,
        DocumentError::PayerMustBeOwner
    );

    let now = Clock::get()?.unix_timestamp;

    // 5) Trim inputs
    let cid_trim = cid_enc.trim();
    let mime_trim = meta_mime.trim();
    let meta_trim = meta_cid.trim();
    let kms_trim = kms_ref.trim();

    let notary_name_trim = notary_name.trim();
    let document_type_trim = document_type.trim();
    let property_ref_trim = property_ref.trim();
    let counterparty_name_trim = counterparty_name.trim();

    // 6) Validate lengths & requireds
    require!(!cid_trim.is_empty(), DocumentError::EmptyCidEnc);
    require!(cid_trim.len() <= MAX_CID_LEN, DocumentError::CidTooLong);

    require!(!mime_trim.is_empty(), DocumentError::EmptyMime);
    require!(
        mime_trim.len() <= MAX_META_MIME_LEN,
        DocumentError::MimeTooLong
    );

    require!(
        meta_trim.len() <= MAX_CID_LEN,
        DocumentError::MetaCidTooLong
    );
    require!(
        kms_trim.len() <= MAX_KMS_REF_LEN,
        DocumentError::KmsRefTooLong
    );

    require!(
        notary_name_trim.len() <= MAX_NOTARY_NAME_LEN,
        DocumentError::NotaryNameTooLong
    );
    require!(
        document_type_trim.len() <= MAX_DOCUMENT_TYPE_LEN,
        DocumentError::DocumentTypeTooLong
    );
    require!(
        property_ref_trim.len() <= MAX_PROPERTY_REF_LEN,
        DocumentError::PropertyRefTooLong
    );
    require!(
        counterparty_name_trim.len() <= MAX_COUNTERPARTY_NAME_LEN,
        DocumentError::CounterpartyNameTooLong
    );

    require!(size_bytes > 0, DocumentError::SizeZero);
    require!(!edek_for_owner.is_empty(), DocumentError::EdekOwnerMissing);
    require!(
        !edek_for_notary.is_empty(),
        DocumentError::EdekNotaryMissing
    );

    if matches!(edek_root_algo, WrapAlgo::Kms) {
        require!(!kms_trim.is_empty(), DocumentError::KmsRefRequired);
    }

    // 7) Sequence
    let owner_seq = &mut ctx.accounts.owner_doc_seq;
    require!(seq == owner_seq.value, DocumentError::BadSeq);
    owner_seq.value = owner_seq
        .value
        .checked_add(1)
        .ok_or(DocumentError::SeqOverflow)?;

    // 8) Fill document
    let doc = &mut ctx.accounts.document;
    doc.owner = ctx.accounts.owner.key();
    doc.notary = ctx.accounts.notary.key();
    doc.uploader = ctx.accounts.uploader.key(); // notary as uploader

    doc.cid_enc = cid_trim.to_string();
    doc.meta_mime = mime_trim.to_string();
    doc.meta_cid = meta_trim.to_string();
    doc.size_bytes = size_bytes;
    doc.blake2b_256 = blake2b_256;

    doc.edek_root = edek_root;
    doc.edek_for_owner = edek_for_owner;
    doc.edek_for_notary = edek_for_notary;
    doc.edek_root_algo = edek_root_algo;
    doc.edek_owner_algo = edek_owner_algo;
    doc.edek_notary_algo = edek_notary_algo;

    doc.kms_ref = kms_trim.to_string();
    doc.seq = seq;
    doc.enc_version = enc_version;
    doc.enc_algo = enc_algo;
    doc.created_at = now;
    doc.updated_at = now;
    doc.bump = ctx.bumps.document;

    // Denorm & extra fields
    doc.owner_pubkey = ctx.accounts.owner.owner_pubkey;
    doc.notary_pubkey = ctx.accounts.notary.authority;

    doc.notary_name = notary_name_trim.to_string();
    doc.document_type = document_type_trim.to_string();
    doc.property_ref = property_ref_trim.to_string();
    doc.counterparty_name = counterparty_name_trim.to_string();

    // 9) Emit
    emit!(DocumentCreated {
        document: doc.key(),
        owner: doc.owner,
        notary: doc.notary,
        uploader: doc.uploader,
        seq,
        enc_version,
        created_at: now,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(seq: u64)]
pub struct CreateDocument<'info> {
    /// Notary signer (initiator / fee payer on the client)
    #[account(mut)]
    pub uploader: Signer<'info>,

    /// Owner must co-sign as payer (rent payer)
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(seeds = [SEED_OWNER, owner.owner_pubkey.as_ref()], bump = owner.bump)]
    pub owner: Account<'info, PropertyOwner>,

    #[account(
        mut,
        seeds = [SEED_OWNER_DOC_SEQ, owner.key().as_ref()],
        bump = owner_doc_seq.bump,
        constraint = owner_doc_seq.owner == owner.key() @ DocumentError::BadSeq
    )]
    pub owner_doc_seq: Account<'info, OwnerDocumentSeq>,

    #[account(seeds = [SEED_NOTARY, notary.authority.as_ref()], bump = notary.bump)]
    pub notary: Account<'info, Notary>,

    #[account(
        seeds = [SEED_GRANT, owner.key().as_ref(), notary.authority.as_ref(), &[SCOPE_WRITE]],
        bump = grant_write.bump,
        constraint = grant_write.owner == owner.key()          @ DocumentError::GrantMismatch,
        constraint = grant_write.grantee == notary.authority   @ DocumentError::GrantMismatch,
        constraint = !grant_write.revoked                      @ DocumentError::GrantRevoked,
    )]
    pub grant_write: Account<'info, Grant>,

    #[account(
        init,
        payer = payer, // rent from owner
        space = 8 + PropertyDocument::INIT_SPACE,
        seeds = [SEED_DOCUMENT, owner.key().as_ref(), &seq.to_le_bytes()],
        bump
    )]
    pub document: Account<'info, PropertyDocument>,

    pub system_program: Program<'info, System>,
}
