use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::{Stream, ProposalStateType, Safe};

#[derive(Accounts)]
#[instruction(account_size : u32)]
pub struct CreateStream<'info> {
    #[account(init, payer = requested_by, space = account_size as usize)]
    stream: Account<'info, Stream>,

    #[account(mut)]
    safe: Account<'info, Safe>,

    #[account(mut)]
    requested_by: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateStream>,
    _account_size: u32,
    client_stream: Stream,
    is_draft: bool,
) -> Result<()> {
    let stream = &mut ctx.accounts.stream;
    let owner = &ctx.accounts.requested_by;
    stream.requested_by = ctx.accounts.requested_by.key();

    let safe = &mut ctx.accounts.safe;
    require!(safe.is_owner(&owner.key()), ErrorCode::InvalidOwner);
    stream.safe = safe.key();
    stream.approvals = Vec::new();
    stream.proposal_stage = if is_draft {
        ProposalStateType::Draft as u8
    } else {
        ProposalStateType::Pending as u8
    };
    stream.owner_set_seqno = safe.owner_set_seqno;

    let now = Clock::get()?.unix_timestamp;
    stream.created_date = now;
    stream.last_updated_date = now;
    stream.apply_stream_data(client_stream, now)?;

    require!(stream.validate_stream_data(), ErrorCode::InvalidJobData);
    Ok(())
}
