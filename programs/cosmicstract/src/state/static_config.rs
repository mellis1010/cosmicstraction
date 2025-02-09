pub enum TriggerType {
    Manual = 1,
    Time = 2,
    Program = 3,
}

pub enum ProposalStateType {
    Draft = 222,
    Pending = 0,
    Approved = 1,
    Rejected = 2,
    ExecutionInProgress = 3,
    Complete = 4,
    Failed = 5, // Future usage
    Aborted = 6,
}

pub enum FeeSource {
    FromFeeAccount = 0,
    FromStream = 1,
}

pub const SAFE_SIGNER_PREFIX: &[u8; 10] = b"SafeSigner";
pub const DEFAULT_STREAM_EXPIRY_DURATION: i64 = 60 * 24 * 60 * 60;
pub const MAXIMUM_REMAINING_RUNS_FOR_PROGRAM_TRIGGER: i16 = 1000;
pub const RECURRING_FOREVER: i16 = -999;
pub const DEFAULT_RETRY_WINDOW: u32 = 300;
pub const STREAM_EXECUTION_FEE: u64 = 50_000;

pub const TIMED_STREAM_COMPLETE: i64 = 0;
pub const TIMED_STREAM_ERROR: i64 = -1;
