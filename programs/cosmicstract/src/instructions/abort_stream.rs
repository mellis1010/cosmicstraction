use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::{Stream, ProposalStateType, Safe};

#[derive(Accounts)]
pub struct AbortStream<'info> {
    #[account(mut, has_one = safe @ErrorCode::InvalidSafe)]
    stream: Account<'info, Stream>,

    safe: Account<'info, Safe>,

    requested_by: Signer<'info>,
}

pub fn handler(ctx: Context<AbortStream>) -> Result<()> {
    let stream = &mut ctx.accounts.stream;
    let safe = &ctx.accounts.safe;
    let caller = &ctx.accounts.requested_by;

    require!(safe.is_owner(&caller.key()), ErrorCode::InvalidOwner);

    require!(
        stream.proposal_stage == ProposalStateType::ExecutionInProgress as u8,
        ErrorCode::RequestIsNotExecutedYet
    );

    let now = Clock::get()?.unix_timestamp;
    stream.proposal_stage = ProposalStateType::Aborted as u8;
    stream.last_updated_date = now;

    Ok(())
}
