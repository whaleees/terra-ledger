use anchor_lang::prelude::*;

pub mod errors;
pub mod event;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("2HZmbrh7agvx2M2z7PjY8U97fpXGuVTouVeVJRvDTXnq");

#[program]
pub mod anchor {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, kms_namespace: String) -> Result<()> {
        config_init(ctx, kms_namespace)
    }

    pub fn register_notaries(
        ctx: Context<RegisterNotaries>,
        name: String,
        kms_ref: String,
    ) -> Result<()> {
        notaries_register(ctx, name, kms_ref)
    }

    pub fn create_document(
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
        document_create(
            ctx,
            seq,
            cid_enc,
            meta_mime,
            meta_cid,
            size_bytes,
            blake2b_256,
            edek_root,
            edek_for_owner,
            edek_for_notary,
            edek_root_algo,
            edek_owner_algo,
            edek_notary_algo,
            kms_ref,
            enc_version,
            enc_algo,
            notary_name,
            document_type,
            property_ref,
            counterparty_name,
        )
    }

    pub fn read_documents(ctx: Context<ReadDocuments>, seq: u64) -> Result<()> {
        documents_read(ctx, seq)
    }

    pub fn upsert_owner(ctx: Context<UpsertOwner>, owner_ref: String) -> Result<()> {
        owner_upsert(ctx, owner_ref)
    }

    pub fn grant_access(ctx: Context<GrantAccess>, scope: u8) -> Result<()> {
        access_grant(ctx, scope)
    }

    pub fn revoke_grant(ctx: Context<RevokeGrant>) -> Result<()> {
        grant_revoke(ctx)
    }

    pub fn set_paused(ctx: Context<SetPaused>, paused: bool) -> Result<()> {
        paused_set(ctx, paused)
    }

    pub fn revoke_delegate(ctx: Context<RevokeDelegate>) -> Result<()> {
        delegate_revoke(ctx)
    }

    pub fn add_delegate(ctx: Context<AddDelegate>) -> Result<()> {
        delegate_add(ctx)
    }
}
