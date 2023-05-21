use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::{Action, Stream, ProposalStateType};

#[derive(Accounts)]
pub struct AddAction<'info> {
    #[account(mut, has_one = requested_by)]
    stream: Account<'info, Stream>,

    requested_by: Signer<'info>,
}

pub fn handler(ctx: Context<AddAction>, client_action: Action, finish_draft: bool) -> Result<()> {
    let stream = &mut ctx.accounts.stream;

    require!(
        stream.approvals.len() == 0,
        ErrorCode::StreamMustHaveZeroApproverBeforeUpdate
    );
    require!(
        stream.proposal_stage == ProposalStateType::Draft as u8,
        ErrorCode::StreamMustBeInDraftedBeforeUpdate
    );

    let now = Clock::get()?.unix_timestamp;
    stream.actions.push(client_action);
    if finish_draft {
        stream.proposal_stage = ProposalStateType::Pending as u8;
    }
    stream.last_updated_date = now;

    Ok(())
}
