use crate::errors::DelegateError;
use crate::event::DelegateRevoked;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn delegate_revoke(ctx: Context<RevokeDelegate>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let delegate_acc = &mut ctx.accounts.delegate_account;

    // Only the property owner themself can revoke.
    require_keys_eq!(
        ctx.accounts.owner.owner_pubkey,
        ctx.accounts.authority.key(),
        DelegateError::Unauthorized
    );

    require!(!delegate_acc.revoked, DelegateError::AlreadyRevoked);

    delegate_acc.revoked = true;
    delegate_acc.revoked_at = Some(now);

    emit!(DelegateRevoked {
        delegate_account: delegate_acc.key(),
        owner: delegate_acc.owner,
        delegate: delegate_acc.delegate,
        revoked_at: now,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RevokeDelegate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [SEED_OWNER, owner.owner_pubkey.as_ref()],
        bump = owner.bump,
    )]
    pub owner: Account<'info, PropertyOwner>,

    #[account(
        mut,
        seeds = [
            SEED_DELEGATE,
            owner.owner_pubkey.as_ref(),
            delegate_account.delegate.as_ref()
        ],
        bump = delegate_account.bump,
        constraint = delegate_account.owner == owner.owner_pubkey @ DelegateError::Unauthorized,
        constraint = !delegate_account.revoked @ DelegateError::AlreadyRevoked
    )]
    pub delegate_account: Account<'info, Delegate>,

    pub system_program: Program<'info, System>,
}
