use crate::event::DelegateAdded;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn delegate_add(ctx: Context<AddDelegate>) -> Result<()> {
    let acc = &mut ctx.accounts.delegate_account;
    acc.owner = ctx.accounts.owner_authority.key();
    acc.delegate = ctx.accounts.delegate.key();
    acc.added_by = ctx.accounts.owner_authority.key();
    acc.created_at = Clock::get()?.unix_timestamp;
    acc.revoked = false;
    acc.revoked_at = None;
    acc.bump = ctx.bumps.delegate_account;

    emit!(DelegateAdded {
        owner: acc.owner,
        delegate: acc.delegate,
        added_at: acc.created_at,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AddDelegate<'info> {
    #[account(mut)]
    pub owner_authority: Signer<'info>,

    #[account(
        seeds = [SEED_OWNER, owner_authority.key().as_ref()],
        bump = owner.bump,
    )]
    pub owner: Account<'info, PropertyOwner>,

    pub delegate: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner_authority,
        space = 8 + Delegate::INIT_SPACE,
        seeds = [SEED_DELEGATE, owner_authority.key().as_ref(), delegate.key().as_ref()],
        bump)]
    pub delegate_account: Account<'info, Delegate>,

    pub system_program: Program<'info, System>,
}
