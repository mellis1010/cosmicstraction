use anchor_lang::prelude::*;

use crate::common::charge_fee;
use crate::error::ErrorCode;
use crate::instructions::{do_execute_multisig_stream, ExecuteMultisigStream};
use crate::state::static_config::ProposalStateType;

pub fn handler<'info>(ctx: Context<ExecuteMultisigStream>, is_successful_run: bool) -> Result<()> {
    validate_scheduled_multisig_stream_before_execute(&ctx)?;
    charge_fee(&ctx)?;

    let now = Clock::get()?.unix_timestamp;
    let stream = &ctx.accounts.stream;
    let mut result = Ok(());

    if is_successful_run {
        require!(
            stream.is_due_for_execute(now),
            ErrorCode::JobIsNotDueForExecution
        );
        result = do_execute_multisig_stream::handler(&ctx);
    } else {
        require!(
            stream.is_schedule_expired(now),
            ErrorCode::CannotMarkJobAsErrorIfItsWithinSchedule
        );
    }

    let stream = &mut ctx.accounts.stream;
    stream.update_after_schedule_run(now, is_successful_run);
    if !stream.has_remaining_runs() {
        stream.proposal_stage = ProposalStateType::Complete as u8;
    }
    stream.last_updated_date = now;

    result
}

pub fn validate_scheduled_multisig_stream_before_execute(
    ctx: &Context<ExecuteMultisigStream>,
) -> Result<()> {
    let stream = &ctx.accounts.stream;

    require!(
        stream.proposal_stage == ProposalStateType::ExecutionInProgress as u8,
        ErrorCode::RequestIsNotExecutedYet
    );

    Ok(())
}
