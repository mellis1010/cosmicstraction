use cosmic_util::scheduler::{CosmicSchedule, CosmicTime};

pub fn calculate_next_execution_time(_cron: &str, utc_offset: i32, now: i64) -> i64 {
    if _cron.trim().is_empty() {
        return 0;
    }

    let local_time = now.checked_sub(utc_offset as i64).unwrap();
    let schedule = CosmicSchedule::parse(_cron).unwrap();
    let next_execution = schedule
        .next_event(&CosmicTime::from_time_ts(local_time))
        .unwrap()
        .to_time_ts(utc_offset)
        .unwrap();
    next_execution
}
