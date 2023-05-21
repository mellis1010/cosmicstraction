pub mod common;
mod error;
pub mod instructions;
pub mod state;
mod test;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("**PLACEHOLDER**");

#[program]
pub mod cosmicstract {
    use super::*;

    pub fn create_stream(
        ctx: Context<CreateStream>,
        account_size: u32,
        client_stream: Stream,
        is_draft: bool,
    ) -> Result<()> {
        instructions::create_stream::handler(ctx, account_size, client_stream, is_draft)
    }

    pub fn delete_stream(_ctx: Context<DeleteStream>) -> Result<()> {
        Ok(())
    }

    pub fn abort_stream(ctx: Context<AbortStream>) -> Result<()> {
        instructions::abort_stream::handler(ctx)
    }

    pub fn create_safe(ctx: Context<CreateSafe>, client_safe: Safe) -> Result<()> {
        instructions::create_safe::handler(ctx, client_safe)
    }

    pub fn add_owner(ctx: Context<AuthSafe>, owner: Pubkey) -> Result<()> {
        instructions::update_safe::add_owner_handler(ctx, owner)
    }

    pub fn remove_owner(ctx: Context<AuthSafe>, owner: Pubkey) -> Result<()> {
        instructions::update_safe::remove_owner_handler(ctx, owner)
    }

    pub fn change_threshold(ctx: Context<AuthSafe>, threshold: u8) -> Result<()> {
        instructions::update_safe::change_threshold_handler(ctx, threshold)
    }

    pub fn approve_proposal(ctx: Context<ApproveProposal>, is_approved: bool) -> Result<()> {
        instructions::approve_proposal::handler(ctx, is_approved)
    }

    pub fn execute_multisig_stream(ctx: Context<ExecuteMultisigStream>) -> Result<()> {
        instructions::execute_multisig_stream::handler(ctx)
    }

    pub fn execute_scheduled_multisig_stream<'info>(ctx: Context<ExecuteMultisigStream>) -> Result<()> {
        instructions::execute_scheduled_multisig_stream::handler(ctx, true)
    }

    pub fn mark_timed_stream_as_error(ctx: Context<ExecuteMultisigStream>) -> Result<()> {
        instructions::execute_scheduled_multisig_stream::handler(ctx, false)
    }

    pub fn add_action(
        ctx: Context<AddAction>,
        client_action: Action,
        finish_draft: bool,
    ) -> Result<()> {
        instructions::add_action::handler(ctx, client_action, finish_draft)
    }
}
