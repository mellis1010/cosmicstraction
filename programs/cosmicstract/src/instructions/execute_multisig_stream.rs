use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::instructions::{do_execute_multisig_stream, ExecuteMultisigStream};
use crate::state::static_config::{ProposalStateType, TriggerType};

pub fn handler(ctx: Context<ExecuteMultisigStream>) -> Result<()> {
    validate_multisig_stream_before_execute(&ctx)?;

    let mut result = Ok(());
    let now = Clock::get()?.unix_timestamp;
    if ctx.accounts.stream.trigger_type == TriggerType::Manual as u8 {
        result = do_execute_multisig_stream::handler(&ctx);
        let stream = &mut ctx.accounts.stream;
        stream.proposal_stage = ProposalStateType::Complete as u8;
        stream.last_updated_date = now;
    } else {
        let stream = &mut ctx.accounts.stream;
        if stream.next_execution_time < now
            && stream.trigger_type == TriggerType::Time as u8
            && stream.recurring
        {
            stream.update_next_execution_time(now);
        }
        stream.proposal_stage = ProposalStateType::ExecutionInProgress as u8;
        stream.last_updated_date = now;
    }

    result
}

pub fn validate_multisig_stream_before_execute(ctx: &Context<ExecuteMultisigStream>) -> Result<()> {
    let safe = &ctx.accounts.safe;
    let stream = &ctx.accounts.stream;
    let caller = &ctx.accounts.caller;
    let execute_by_safe_owner = safe.is_owner(&caller.key());

    require!(execute_by_safe_owner, ErrorCode::InvalidOwner);
    require!(
        stream.proposal_stage == ProposalStateType::Approved as u8,
        ErrorCode::RequestIsNotApprovedYet
    );

    let now = Clock::get()?.unix_timestamp;
    require!(now <= stream.expiry_date, ErrorCode::JobIsExpired);

    Ok(())
}
