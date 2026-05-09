use crate::errors::{AccessError, DelegateError};
use crate::event::{DelegateGrantCreated, GrantCreated};
use crate::states::*;
use anchor_lang::prelude::*;

pub fn access_grant(ctx: Context<GrantAccess>, scope: u8) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let authority = ctx.accounts.authority.key();
    let owner_pk = ctx.accounts.owner.owner_pubkey;

    // Determine if caller is owner or delegate.
    let is_owner = authority == owner_pk;
    let delegate_acc_opt = &ctx.accounts.delegate_account;
    let is_delegate = delegate_acc_opt.is_some();

    require!(is_owner || is_delegate, AccessError::UnauthorizedGrant);

    if let Some(delegate_acc) = delegate_acc_opt.as_ref() {
        require_keys_eq!(delegate_acc.owner, owner_pk, DelegateError::Unauthorized);
        require_keys_eq!(
            delegate_acc.delegate,
            authority,
            DelegateError::Unauthorized
        );
        require!(!delegate_acc.revoked, DelegateError::Revoked);

        require!(scope == SCOPE_READ, DelegateError::ReadOnly);
    }

    let allowed = SCOPE_READ | SCOPE_WRITE | SCOPE_ADMIN;
    require!(
        scope != 0 && (scope & !allowed) == 0,
        AccessError::InvalidScope
    );

    let grant = &mut ctx.accounts.grant;
    grant.owner = ctx.accounts.owner.key();
    grant.grantee = ctx.accounts.grantee.key();
    grant.scope = scope;
    grant.created_by = authority;
    grant.created_at = now;
    grant.via_delegate = is_delegate;
    grant.revoked = false;
    grant.revoked_at = None;
    grant.bump = ctx.bumps.grant;

    if is_delegate {
        emit!(DelegateGrantCreated {
            grant: grant.key(),
            owner: grant.owner,
            grantee: grant.grantee,
            delegate: authority,
            scope,
            created_at: now,
        });
    } else {
        emit!(GrantCreated {
            grant: grant.key(),
            owner: grant.owner,
            grantee: grant.grantee,
            scope,
            created_by: authority,
            created_at: now,
        });
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(scope: u8)]
pub struct GrantAccess<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        constraint = !config.paused @ AccessError::Paused
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [SEED_OWNER, owner.owner_pubkey.as_ref()],
        bump = owner.bump,
    )]
    pub owner: Account<'info, PropertyOwner>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + Grant::INIT_SPACE,
        seeds = [SEED_GRANT, owner.key().as_ref(), grantee.key().as_ref(), &[scope]],
        bump
    )]
    pub grant: Account<'info, Grant>,

    pub grantee: SystemAccount<'info>,

    pub delegate_account: Option<Account<'info, Delegate>>,

    pub system_program: Program<'info, System>,
}
