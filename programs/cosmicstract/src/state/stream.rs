use anchor_lang::prelude::*;

use crate::common::calculate_next_execution_time;
use crate::error::ErrorCode;
use crate::state::{
    Action, ApprovalRecord, TriggerType, DEFAULT_STREAM_EXPIRY_DURATION, DEFAULT_RETRY_WINDOW,
    MAXIMUM_REMAINING_RUNS_FOR_PROGRAM_TRIGGER, RECURRING_FOREVER, TIMED_STREAM_COMPLETE,
    TIMED_STREAM_ERROR,
};
use cosmic_util::scheduler::is_valid_utc_offset;

#[account]
#[derive(Debug)]
pub struct Stream {
    pub requested_by: Pubkey,
    pub safe: Pubkey,
    pub last_updated_date: i64,
    pub proposal_stage: u8,
    pub created_date: i64,
    pub trigger_type: u8,
    pub next_execution_time: i64,
    pub retry_window: u32,
    pub recurring: bool,
    pub remaining_runs: i16,
    pub schedule_end_date: i64,
    pub client_app_id: u32,
    pub last_rent_charged: i64,
    pub last_scheduled_execution: i64,
    pub expiry_date: i64,
    pub expire_on_complete: bool,
    pub app_id: Pubkey,
    pub pay_fee_from: u8,
    pub user_utc_offset: i32,
    pub custom_compute_budget: u32,
    pub custom_fee: u32,
    pub custom_field_1: i32,
    pub custom_field_2: i32,
    pub owner_set_seqno: u8,
    pub external_id: String,
    pub cron: String,
    pub name: String,
    pub extra: String,
    pub actions: Vec<Action>,
    pub approvals: Vec<ApprovalRecord>,
}

impl Stream {
    pub fn apply_stream_data(&mut self, client_stream: Stream, now: i64) -> Result<()> {
        require!(
            is_valid_utc_offset(client_stream.user_utc_offset),
            ErrorCode::InvalidUtcOffset
        );
        if client_stream.recurring {
            require!(
                !client_stream.cron.trim().is_empty(),
                ErrorCode::InvalidCronPatternForScheduledStream
            );
        }

        self.trigger_type = client_stream.trigger_type;
        self.recurring = client_stream.recurring;
        self.remaining_runs = client_stream.remaining_runs;
        self.retry_window = client_stream.retry_window;
        self.cron = client_stream.cron;
        self.name = client_stream.name;
        self.actions = client_stream.actions;
        self.user_utc_offset = client_stream.user_utc_offset;
        self.pay_fee_from = client_stream.pay_fee_from;
        self.client_app_id = client_stream.client_app_id;
        self.external_id = client_stream.external_id;
        self.custom_compute_budget = client_stream.custom_compute_budget;
        self.custom_fee = client_stream.custom_fee;
        self.app_id = client_stream.app_id;
        self.schedule_end_date = client_stream.schedule_end_date;
        self.expiry_date = if client_stream.expiry_date > now {
            client_stream.expiry_date
        } else {
            now.checked_add(DEFAULT_STREAM_EXPIRY_DURATION).unwrap()
        };
        self.expire_on_complete = false;
        self.extra = client_stream.extra;

        if self.trigger_type == TriggerType::Time as u8 {
            if self.retry_window < 1 {
                self.retry_window = DEFAULT_RETRY_WINDOW;
            }

            if self.recurring {
                if self.has_remaining_runs() {
                    if client_stream.next_execution_time == 0 {
                        self.update_next_execution_time(now);
                    } else {
                        self.next_execution_time = client_stream.next_execution_time;
                    }
                } else {
                    self.next_execution_time = TIMED_STREAM_COMPLETE;
                }
            } else {
                self.next_execution_time = client_stream.next_execution_time;
                self.remaining_runs = 1;
            }
        } else if self.trigger_type == TriggerType::Program as u8 {
            require!(
                client_stream.remaining_runs >= 0
                    && client_stream.remaining_runs <= MAXIMUM_REMAINING_RUNS_FOR_PROGRAM_TRIGGER,
                ErrorCode::InvalidRemainingRuns
            );
        }
        Ok(())
    }

    pub fn get_approvals(&self) -> u8 {
        self.approvals
            .iter()
            .filter(|approval| approval.is_approved)
            .count() as u8
    }

    pub fn is_new_owner_approval(&self, owner: &Pubkey) -> bool {
        self.approvals
            .iter()
            .all(|approval| approval.owner != *owner)
    }

    pub fn validate_stream_data(&self) -> bool {
        if self.trigger_type != TriggerType::Manual as u8
            && self.trigger_type != TriggerType::Time as u8
            && self.trigger_type != TriggerType::Program as u8
        {
            return false;
        }

        if self.remaining_runs == RECURRING_FOREVER && self.recurring == false {
            return false;
        }

        if self.remaining_runs < 0 && self.remaining_runs != RECURRING_FOREVER {
            return false;
        }
        true
    }

    pub fn has_remaining_runs(&self) -> bool {
        self.remaining_runs > 0 || self.remaining_runs == RECURRING_FOREVER
    }

    pub fn is_due_for_execute(&self, now: i64) -> bool {
        if self.trigger_type == TriggerType::Program as u8 {
            return self.has_remaining_runs();
        }

        if self.trigger_type == TriggerType::Time as u8 {
            return self.next_execution_time > 0
                && self.next_execution_time < now
                && now - self.next_execution_time < self.retry_window as i64;
        }

        false
    }

    pub fn is_schedule_expired(&self, now: i64) -> bool {
        return self.trigger_type == TriggerType::Time as u8
            && self.next_execution_time > 0
            && now.checked_sub(self.next_execution_time).unwrap() > self.retry_window as i64;
    }

    pub fn update_after_schedule_run(&mut self, now: i64, is_successful_run: bool) {
        self.last_scheduled_execution = now;
        if self.remaining_runs != RECURRING_FOREVER {
            self.remaining_runs = self.remaining_runs.checked_sub(1).unwrap();
        }

        if self.trigger_type == TriggerType::Time as u8 {
            if self.has_remaining_runs() {
                self.update_next_execution_time(now);
            } else {
                self.next_execution_time = if is_successful_run {
                    TIMED_STREAM_COMPLETE
                } else {
                    TIMED_STREAM_ERROR
                };
            }
        }

        self.last_updated_date = now;
    }

    pub fn update_next_execution_time(&mut self, now: i64) {
        self.next_execution_time =
            calculate_next_execution_time(&self.cron, self.user_utc_offset, now);
    }
}
