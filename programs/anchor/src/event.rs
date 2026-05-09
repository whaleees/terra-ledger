use anchor_lang::prelude::*;

#[event]
pub struct ConfigInitialized {
    pub authority: Pubkey,
    pub kms_namespace: String,
    pub created_at: i64,
}

#[event]
pub struct GrantUpdated {
    pub owner: Pubkey,
    pub grantee: Pubkey,
    pub revoked: bool,
}

#[event]
pub struct NotaryRegistered {
    pub notary: Pubkey,
    pub notary_authority: Pubkey,
    pub name: String,
    pub kms_ref: String,
    pub registered_by: Pubkey,
    pub created_at: i64,
}

#[event]
pub struct OwnerUpserted {
    pub owner: Pubkey,
    pub owner_pubkey: Pubkey,
    pub created: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[event]
pub struct ProgramPauseUpdated {
    pub paused: bool,
    pub set_by: Pubkey,
    pub at: i64,
}

#[event]
pub struct DocumentCreated {
    pub document: Pubkey,
    pub owner: Pubkey,
    pub notary: Pubkey,
    pub uploader: Pubkey,
    pub seq: u64,
    pub enc_version: u16,
    pub created_at: i64,
}

#[event]
pub struct DocumentRead {
    pub document: Pubkey,
    pub owner: Pubkey,
    pub notary: Pubkey,
    pub reader: Pubkey,
    pub seq: u64,
}

#[event]
pub struct GrantCreated {
    pub grant: Pubkey,
    pub owner: Pubkey,
    pub grantee: Pubkey,
    pub scope: u8,
    pub created_by: Pubkey,
    pub created_at: i64,
}

#[event]
pub struct GrantRevoked {
    pub grant: Pubkey,
    pub owner: Pubkey,
    pub grantee: Pubkey,
    pub scope: u8,
    pub revoked_by: Pubkey,
    pub revoked_at: i64,
}

#[event]
pub struct DelegateAdded {
    pub owner: Pubkey,
    pub delegate: Pubkey,
    pub added_at: i64,
}

#[event]
pub struct DelegateGrantCreated {
    pub grant: Pubkey,
    pub owner: Pubkey,
    pub grantee: Pubkey,
    pub delegate: Pubkey,
    pub scope: u8,
    pub created_at: i64,
}

#[event]
pub struct DelegateRevoked {
    pub delegate_account: Pubkey,
    pub owner: Pubkey,
    pub delegate: Pubkey,
    pub revoked_at: i64,
}
