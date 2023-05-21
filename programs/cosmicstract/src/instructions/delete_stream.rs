use anchor_lang::prelude::*;

use crate::state::Stream;
use crate::state::ProposalStateType;

#[derive(Accounts)]
pub struct DeleteStream<'info> {
    #[account(
        mut,
        has_one = requested_by,
        close = requested_by,
        constraint =
            stream.proposal_stage != ProposalStateType::ExecutionInProgress as u8
            && stream.proposal_stage != ProposalStateType::Complete as u8
            && stream.proposal_stage != ProposalStateType::Aborted as u8
    )]
    stream: Account<'info, Stream>,

    requested_by: Signer<'info>,
}
