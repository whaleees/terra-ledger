use anchor_lang::prelude::*;

#[error_code]
pub enum ConfigError {
    #[msg("Config already initialized")]
    AlreadyInitialized,
    #[msg("KMS namespace is empty")]
    EmptyKmsNamespace,
    #[msg("KMS namespace too long")]
    KmsNamespaceTooLong,
    #[msg("Program is paused; action not allowed")]
    Paused,
    #[msg("Only config authority may pause/unpause the program")]
    UnauthorizedPauseAction,
}

#[error_code]
pub enum NotaryError {
    #[msg("Program is paused; action not allowed")]
    Paused,
    #[msg("Only config authority may register notaries")]
    UnauthorizedRegistrar,
    #[msg("Notary name is empty")]
    EmptyName,
    #[msg("Notary name too long")]
    NameTooLong,
    #[msg("KMS reference is empty")]
    EmptyKmsRef,
    #[msg("KMS reference too long")]
    KmsRefTooLong,
}

#[error_code]
pub enum OwnerError {
    #[msg("Unauthorized: only the property owner wallet may update this account")]
    Unauthorized,
    #[msg("Owner reference too long")]
    OwnerRefTooLong,
}

#[error_code]
pub enum DocumentError {
    #[msg("Program is paused")]
    Paused,
    #[msg("Uploader must be the notary authority")]
    UploaderNotNotaryAuthority,
    #[msg("WRITE grant is revoked")]
    GrantRevoked,
    #[msg("WRITE grant is expired")]
    GrantExpired,
    #[msg("WRITE grant does not match owner/notary")]
    GrantMismatch,
    #[msg("Bad sequence number")]
    BadSeq,
    #[msg("Sequence overflow")]
    SeqOverflow,
    #[msg("cid_enc is empty")]
    EmptyCidEnc,
    #[msg("meta_mime is empty")]
    EmptyMime,
    #[msg("size_bytes must be > 0")]
    SizeZero,
    #[msg("kms_ref must be non-empty when edek_root_algo=Kms")]
    KmsRefRequired,
    #[msg("edek_for_owner is empty")]
    EdekOwnerMissing,
    #[msg("edek_for_notary is empty")]
    EdekNotaryMissing,
    #[msg("cid_enc too long")]
    CidTooLong,
    #[msg("meta_mime too long")]
    MimeTooLong,
    #[msg("meta_cid too long")]
    MetaCidTooLong,
    #[msg("note too long")]
    NoteTooLong,
    #[msg("kms_ref too long")]
    KmsRefTooLong,

    #[msg("Notary name too long")]
    NotaryNameTooLong,
    #[msg("Document type too long")]
    DocumentTypeTooLong,
    #[msg("Property reference too long")]
    PropertyRefTooLong,
    #[msg("Counterparty name too long")]
    CounterpartyNameTooLong,
    #[msg("Diagnosis too long")]
    DiagnosisTooLong,
    #[msg("Keywords too long")]
    KeywordsTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
    #[msg("Payer must be property owner")]
    PayerMustBeOwner,
    #[msg("Document linkage mismatch")]
    DocumentMismatch,
}

#[error_code]
pub enum AccessError {
    #[msg("Program is paused")]
    Paused,
    #[msg("Only the property owner may create a grant")]
    UnauthorizedGrant,
    #[msg("Scope must only include READ/WRITE/ADMIN bits and not be zero")]
    InvalidScope,
    #[msg("Expiry must be in the future")]
    BadExpiry,
    #[msg("Grant already revoked")]
    AlreadyRevoked,
    #[msg("Only the property owner may revoke this grant")]
    UnauthorizedRevoke,
}

#[error_code]
pub enum DelegateError {
    #[msg("Delegate is already revoked")]
    AlreadyRevoked,

    #[msg("Unauthorized action: only owner can modify delegates")]
    Unauthorized,

    #[msg("Delegate not found")]
    NotFound,

    #[msg("Delegate is revoked")]
    Revoked,

    #[msg("Delegate can only issue READ access")]
    ReadOnly,

    #[msg("Invalid scope for delegate")]
    InvalidScope,
}
