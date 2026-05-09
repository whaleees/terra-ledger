use anchor_lang::prelude::*;

pub const SEED_CONFIG: &[u8] = b"config";
pub const SEED_NOTARY: &[u8] = b"notary";
pub const SEED_OWNER: &[u8] = b"owner";
pub const SEED_DOCUMENT: &[u8] = b"document";
pub const SEED_GRANT: &[u8] = b"grant";
pub const SEED_OWNER_DOC_SEQ: &[u8] = b"owner_doc_seq";
pub const SEED_CONSENT: &[u8] = b"consent";
pub const SEED_ACCESS: &[u8] = b"access";
pub const SEED_DELEGATE: &[u8] = b"delegate";

pub const MAX_NAME_LEN: usize = 100;
pub const MAX_CID_LEN: usize = 100;
pub const MAX_NOTE_LEN: usize = 150;
pub const MAX_KMS_REF_LEN: usize = 64;
pub const MAX_META_MIME_LEN: usize = 40;
pub const MAX_OWNER_REF_LEN: usize = 128;
pub const MAX_WRAP_DEK_LEN: usize = 256;

pub const SCOPE_READ: u8 = 0b0000_0001;
pub const SCOPE_WRITE: u8 = 0b0000_0010;
pub const SCOPE_ADMIN: u8 = 0b0000_0100;

pub const MAX_NOTARY_NAME_LEN: usize = 128;
pub const MAX_DOCUMENT_TYPE_LEN: usize = 128;
pub const MAX_PROPERTY_REF_LEN: usize = 128;
pub const MAX_COUNTERPARTY_NAME_LEN: usize = 128;

pub const MAX_DIAGNOSIS_LEN: usize = 256;
pub const MAX_KEYWORDS_LEN: usize = 256;
pub const MAX_DESCRIPTION_LEN: usize = 1024;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq)]
pub enum WrapAlgo {
    Kms,
    SealedBox,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq)]
pub enum EncAlgo {
    XChaCha20,
    AesGcm,
    Other,
}

/// Program Config
/// PDA: [SEED_CONFIG]
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub paused: bool,

    #[max_len(MAX_KMS_REF_LEN)]
    pub kms_namespace: String,

    pub bump: u8,
}

/// Registered notary or verification office.
/// PDA: [SEED_NOTARY, notary_authority]
#[account]
#[derive(InitSpace)]
pub struct Notary {
    pub authority: Pubkey,

    #[max_len(MAX_NAME_LEN)]
    pub name: String,

    #[max_len(MAX_KMS_REF_LEN)]
    pub kms_ref: String,

    pub registered_by: Pubkey,

    pub created_at: i64,

    pub bump: u8,
}

/// Property owner profile.
/// PDA: [SEED_OWNER, owner_pubkey]
#[account]
#[derive(InitSpace)]
pub struct PropertyOwner {
    pub owner_pubkey: Pubkey,

    #[max_len(MAX_OWNER_REF_LEN)]
    pub owner_ref: String,

    pub created_at: i64,

    pub bump: u8,
}

/// PDA: [SEED_OWNER_DOC_SEQ, owner]
#[account]
pub struct OwnerDocumentSeq {
    pub owner: Pubkey,
    pub value: u64,
    pub bump: u8,
}

/// Immutable property document metadata.
/// PDA: [SEED_DOCUMENT, owner, seq_le_bytes]
#[account]
#[derive(InitSpace)]
pub struct PropertyDocument {
    pub owner: Pubkey,
    pub notary: Pubkey,
    pub uploader: Pubkey,

    #[max_len(MAX_CID_LEN)]
    pub cid_enc: String,

    #[max_len(MAX_META_MIME_LEN)]
    pub meta_mime: String, // file type

    #[max_len(MAX_CID_LEN)]
    pub meta_cid: String, // metadata json (document_seq, created_at, ...)

    pub size_bytes: u64,

    pub blake2b_256: [u8; 32],

    #[max_len(MAX_WRAP_DEK_LEN)]
    pub edek_root: Vec<u8>, // ini kalau mau ada centralized authority, pake root kalo gak hapus

    #[max_len(MAX_WRAP_DEK_LEN)]
    pub edek_for_owner: Vec<u8>,

    #[max_len(MAX_WRAP_DEK_LEN)]
    pub edek_for_notary: Vec<u8>,

    pub edek_root_algo: WrapAlgo,
    pub edek_owner_algo: WrapAlgo,
    pub edek_notary_algo: WrapAlgo,

    #[max_len(MAX_KMS_REF_LEN)]
    pub kms_ref: String,

    pub seq: u64,

    pub enc_version: u16,
    pub enc_algo: EncAlgo,

    pub created_at: i64,
    pub updated_at: i64,

    pub bump: u8,

    /// The owner's main wallet pubkey (denormalized from PropertyOwner account)
    pub owner_pubkey: Pubkey,

    /// The notary's authority pubkey (denormalized from Notary account)
    pub notary_pubkey: Pubkey,

    #[max_len(MAX_NOTARY_NAME_LEN)]
    pub notary_name: String,

    #[max_len(MAX_DOCUMENT_TYPE_LEN)]
    pub document_type: String,

    #[max_len(MAX_PROPERTY_REF_LEN)]
    pub property_ref: String,

    #[max_len(MAX_COUNTERPARTY_NAME_LEN)]
    pub counterparty_name: String,
}

/// PDA: [SEED_ACCESS, owner, document, grantee]
#[account]
#[derive(InitSpace)]
pub struct AccessEnvelope {
    pub owner: Pubkey,
    pub document: Pubkey,
    pub grantee: Pubkey,

    #[max_len(MAX_WRAP_DEK_LEN)]
    pub edek_for_grantee: Vec<u8>,
    pub edek_grantee_algo: WrapAlgo,

    #[max_len(MAX_KMS_REF_LEN)]
    pub grantee_kms_ref: String,

    pub created_at: i64,
    pub bump: u8,
}

/// Grant (Per-Grantee)
/// PDA: [SEED_GRANT, owner, grantee, scope_byte]
#[account]
#[derive(InitSpace)]
pub struct Grant {
    pub owner: Pubkey,
    pub grantee: Pubkey,
    pub scope: u8,

    pub created_by: Pubkey,
    pub created_at: i64,
    pub via_delegate: bool,

    pub revoked: bool,
    pub revoked_at: Option<i64>,

    pub bump: u8,
}

/// PDA: [b"consent", owner, document, grantee, nonce16]
/// Urgent only
#[account]
#[derive(InitSpace)]
pub struct Consent {
    pub owner: Pubkey,
    pub document: Pubkey,

    pub grantee: Pubkey,

    pub scope: u8,

    #[max_len(MAX_KMS_REF_LEN)]
    pub kms_ref: String,

    pub edek_root_digest: [u8; 32],

    pub nonce16: [u8; 16],
    pub expires_at: i64,

    pub used: bool, //default = false, kalo sudah dec, used = true.

    pub created_at: i64,
    pub bump: u8,
}

/// PDA: [SEED_DELEGATE, owner, delegate]
#[account]
#[derive(InitSpace)]
pub struct Delegate {
    pub owner: Pubkey,
    pub delegate: Pubkey,
    pub added_by: Pubkey,
    pub created_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub bump: u8,
}
