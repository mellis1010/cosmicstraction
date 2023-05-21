#[cfg(test)]
mod tests {
    use anchor_lang::prelude::*;

    use crate::state::approval_record::ApprovalRecord;
    use crate::state::static_config::*;
    use crate::state::Stream;

    #[test]
    fn test_apply_stream_data() {
        let now = 1644466423;

        let mut stream = sample_recurring_timed_stream();
        let client_stream = sample_recurring_timed_stream();
        assert!(stream.apply_stream_data(client_stream, now).is_ok());

        let mut stream = sample_recurring_timed_stream();
        let mut client_stream = sample_recurring_timed_stream();
        client_stream.cron = String::new();
        assert!(stream.apply_stream_data(client_stream, now).is_err());
    }

    #[test]
    fn test_update_schedule_for_a_recurring_timed_stream_after_a_successful_run() {
        let mut stream = sample_recurring_timed_stream();
        let now = 1644466423;

        stream.update_after_schedule_run(now, true);

        assert_eq!(stream.remaining_runs, 2);
        assert_eq!(stream.next_execution_time, 1646089200);
    }

    #[test]
    fn test_update_schedule_for_a_recurring_timed_stream_after_the_last_successful_run() {
        let mut stream = sample_recurring_timed_stream();
        let now = 1644466423;
        stream.remaining_runs = 1;

        stream.update_after_schedule_run(now, true);

        assert_eq!(stream.remaining_runs, 0);
        assert_eq!(stream.next_execution_time, TIMED_STREAM_COMPLETE);
    }

    #[test]
    fn test_update_schedule_for_a_recurring_timed_stream_after_the_last_error_run() {
        let mut stream = sample_recurring_timed_stream();
        let now = 1644466423;
        stream.remaining_runs = 1;

        stream.update_after_schedule_run(now, false);

        assert_eq!(stream.remaining_runs, 0);
        assert_eq!(stream.next_execution_time, TIMED_STREAM_ERROR);
    }

    #[test]
    fn test_update_schedule_for_an_once_off_timed_stream_after_an_error_run() {
        let mut stream = sample_recurring_timed_stream();
        let now = 1644466423;
        stream.remaining_runs = 1;
        stream.recurring = false;

        stream.update_after_schedule_run(now, false);

        assert_eq!(stream.remaining_runs, 0);
        assert_eq!(stream.next_execution_time, TIMED_STREAM_ERROR);
        assert_eq!(stream.last_scheduled_execution, now);
        assert_eq!(stream.last_updated_date, now);
    }

    #[test]
    fn test_update_schedule_for_an_once_off_timed_stream_after_a_successful_run() {
        let mut stream = sample_recurring_timed_stream();
        let now = 1644466423;
        stream.remaining_runs = 1;
        stream.recurring = false;

        stream.update_after_schedule_run(now, true);

        assert_eq!(stream.remaining_runs, 0);
        assert_eq!(stream.next_execution_time, TIMED_STREAM_COMPLETE);
        assert_eq!(stream.last_scheduled_execution, now);
        assert_eq!(stream.last_updated_date, now);
    }

    #[test]
    fn test_update_schedule_for_a_conditional_stream() {
        let mut stream = sample_recurring_timed_stream();
        let now = 1644466423;
        stream.remaining_runs = 1;
        stream.trigger_type = TriggerType::Program as u8;

        stream.update_after_schedule_run(now, true);

        assert_eq!(stream.remaining_runs, 0);
        assert_eq!(stream.next_execution_time, TIMED_STREAM_COMPLETE);
        assert_eq!(stream.last_scheduled_execution, now);
        assert_eq!(stream.last_updated_date, now);
    }

    #[test]
    fn test_approvals() {
        let mut stream = sample_recurring_timed_stream();
        let owner_a = Pubkey::new_unique();

        assert_eq!(stream.is_new_owner_approval(&owner_a), true);

        let owner_b = Pubkey::new_unique();
        let owner_c = Pubkey::new_unique();
        stream.approvals = vec![ApprovalRecord {
            owner: owner_b,
            date: 1652937049,
            is_approved: false,
        }];
        assert_eq!(stream.is_new_owner_approval(&owner_a), true);
        assert_eq!(stream.is_new_owner_approval(&owner_b), false);
        assert_eq!(stream.is_new_owner_approval(&owner_c), true);
    }

    #[test]
    fn test_calculate_next_execution_time() {
        let mut stream = sample_recurring_timed_stream();
        stream.remaining_runs = 100;
        stream.user_utc_offset = -36000;
        let now = 1661400000;

        stream.cron = String::from("0 15 * * 1-5/2");
        stream.next_execution_time = 0;
        stream.update_next_execution_time(now);
        assert_eq!(stream.next_execution_time, 1661490000);

        stream.cron = String::from("0 15 * * 2-6/2");
        stream.next_execution_time = 0;
        stream.update_next_execution_time(now);
        assert_eq!(stream.next_execution_time, 1661403600);
    }

    fn sample_recurring_timed_stream() -> Stream {
        Stream {
            requested_by: Pubkey::new_unique(),
            last_updated_date: 0,
            created_date: 0,
            trigger_type: TriggerType::Time as u8,
            next_execution_time: 0,
            retry_window: 0,
            recurring: true,
            remaining_runs: 3,
            schedule_end_date: 0,
            client_app_id: 0,
            last_rent_charged: 0,
            last_scheduled_execution: 0,
            expiry_date: 0,
            expire_on_complete: false,
            app_id: Pubkey::new_unique(),
            pay_fee_from: 0,
            user_utc_offset: -39600,
            custom_compute_budget: 0,
            custom_fee: 0,
            custom_field_1: 0,
            custom_field_2: 0,
            external_id: "".to_string(),
            cron: String::from("0 10 1 * *"),
            name: "".to_string(),
            extra: "".to_string(),
            actions: vec![],
            safe: Pubkey::new_unique(),
            approvals: vec![],
            proposal_stage: 0,
            owner_set_seqno: 0,
        }
    }
}
