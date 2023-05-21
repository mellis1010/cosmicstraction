use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Cosmicstract: The job data is invalid.")]
    InvalidJobData,

    #[msg("Cosmicstract: Invalid execution type for the job.")]
    InvalidExecutionType,

    #[msg("Cosmicstract: The job is not due for execution.")]
    JobIsNotDueForExecution,

    #[msg("Cosmicstract: The job is expired.")]
    JobIsExpired,

    #[msg("Cosmicstract: Unable to mark the time triggered job as error because it is still within schedule.")]
    CannotMarkJobAsErrorIfItsWithinSchedule,

    #[msg("Cosmicstract: User instruction must not reference the node operator.")]
    UserInstructionMustNotReferenceTheNodeOperator,

    #[msg("Cosmicstract: Creator is not assigned to owner list")]
    CreatorIsNotAssignedToOwnerList,

    #[msg("Cosmicstract: At least 1 required approvals")]
    InvalidMinApprovalsRequired,

    #[msg("Cosmicstract: Required approvals exceeds the number of owners")]
    InvalidMaxApprovalsRequired,

    #[msg("Cosmicstract: At least 1 owner.")]
    InvalidMinOwnerCount,

    #[msg("Cosmicstract: Max owner reached.")]
    InvalidMaxOwnerCount,

    #[msg("Cosmicstract: Invalid Safe.")]
    InvalidSafe,

    #[msg("Cosmicstract: Not an owner.")]
    InvalidOwner,

    #[msg("Cosmicstract: Duplicate owner address in safe.")]
    DuplicateOwnerInSafe,

    #[msg("Cosmicstract: Owner is not removed from safe")]
    OwnerIsNotRemoved,

    #[msg("Cosmicstract: Address signed already")]
    AddressSignedAlready,

    #[msg("Cosmicstract: Request is rejected")]
    RequestIsRejected,

    #[msg("Cosmicstract: Request is executed already")]
    RequestIsExecutedAlready,

    #[msg("Cosmicstract: Request is not approved yet")]
    RequestIsNotApprovedYet,

    #[msg("Cosmicstract: Request is not executed yet")]
    RequestIsNotExecutedYet,

    #[msg("Cosmicstract: Exceed limit proposal signatures")]
    ExceedLimitProposalSignatures,

    #[msg("Cosmicstract: Stream not enough approvals")]
    StreamNotEnoughApprovals,

    #[msg("Cosmicstract: Stream must have zero approver before update")]
    StreamMustHaveZeroApproverBeforeUpdate,

    #[msg("Cosmicstract: Stream must be in drafted before update")]
    StreamMustBeInDraftedBeforeUpdate,

    #[msg("Cosmicstract: Stream is not ready yet")]
    StreamIsNotReadyYet,

    #[msg("Cosmicstract: Invalid UTC offset")]
    InvalidUtcOffset,

    #[msg("Cosmicstract: Cron pattern cannot be empty for scheduled stream")]
    InvalidCronPatternForScheduledStream,

    #[msg("Cosmicstract: Remaining run must be between 0 and 1000")]
    InvalidRemainingRuns,
}
