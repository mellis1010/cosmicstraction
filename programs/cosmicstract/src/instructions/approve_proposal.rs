use crate::error::ErrorCode;
use crate::state::{ApprovalRecord, Stream, ProposalStateType, Safe};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveProposal<'info> {
    #[account(constraint = safe.owner_set_seqno == stream.owner_set_seqno)]
    safe: Account<'info, Safe>,

    #[account(mut, has_one = safe @ErrorCode::InvalidSafe)]
    stream: Account<'info, Stream>,

    #[account(mut)]
    caller: Signer<'info>,
}

pub fn handler(ctx: Context<ApproveProposal>, is_approved: bool) -> Result<()> {
    let stream = &mut ctx.accounts.stream;
    let safe = &ctx.accounts.safe;
    let caller = &mut ctx.accounts.caller;
    let total_owners = safe.owners.len() as u8;

    require!(safe.is_owner(&caller.key()), ErrorCode::InvalidOwner);
    require!(
        stream.approvals.len() < total_owners as usize,
        ErrorCode::ExceedLimitProposalSignatures
    );

    require!(
        stream.is_new_owner_approval(&caller.key()),
        ErrorCode::AddressSignedAlready
    );

    require!(
        stream.proposal_stage == ProposalStateType::Pending as u8,
        ErrorCode::StreamIsNotReadyYet
    );

    let now = Clock::get()?.unix_timestamp;
    require!(now <= stream.expiry_date, ErrorCode::JobIsExpired);

    stream.approvals.push(ApprovalRecord {
        date: now,
        is_approved,
        owner: *caller.to_account_info().key,
    });

    let approvals = stream.get_approvals();
    let unsigned_owners = total_owners
        .checked_sub(stream.approvals.len() as u8)
        .unwrap();
    if safe.approvals_required.checked_sub(approvals).unwrap() > unsigned_owners {
        stream.proposal_stage = ProposalStateType::Rejected as u8;
    }

    if approvals == safe.approvals_required {
        stream.proposal_stage = ProposalStateType::Approved as u8;
    }
    stream.last_updated_date = now;

    Ok(())
}
