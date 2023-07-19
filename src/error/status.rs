use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

/// Manages a default static list of error reasons
/// This can be overriden to a custom list of error reasons
/// when using [`crate::ErrorInfo`](crate::ErrorInfo)
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, Serialize, Deserialize)]
pub enum Status {
    /// The operation completed successfully.
    Ok = 0,

    /// The operation was cancelled.
    Cancelled = 1,

    /// Unknown error.
    Unknown = 2,

    /// Client specified an invalid argument.
    InvalidArgument = 3,

    /// Deadline expired before operation could complete.
    DeadlineExceeded = 4,

    /// Some requested entity was not found.
    NotFound = 5,

    /// Some entity that we attempted to create already exists.
    AlreadyExists = 6,

    /// The caller does not have permission to execute the specified operation.
    PermissionDenied = 7,

    /// Some resource has been exhausted.
    ResourceExhausted = 8,

    /// The system is not in a state required for the operation's execution.
    FailedPrecondition = 9,

    /// The operation was aborted.
    Aborted = 10,

    /// Operation was attempted past the valid range.
    OutOfRange = 11,

    /// Operation is not implemented or not supported.
    Unimplemented = 12,

    /// Internal error.
    Internal = 13,

    /// The service is currently unavailable.
    Unavailable = 14,

    /// Unrecoverable data loss or corruption.
    DataLoss = 15,

    /// The request does not have valid authentication credentials
    Unauthenticated = 16,

    /// Database error occured while processing
    DatabaseError = 17,

    /// Messaging queue/comms error occured while processing (e.g. NATS error)
    MessagingError = 18,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Ok => write!(f, "Ok"),
            Status::Cancelled => write!(f, "CANCELLED"),
            Status::Unknown => write!(f, "UNKNOWN"),
            Status::InvalidArgument => write!(f, "INVALID_ARGUMENT"),
            Status::DeadlineExceeded => write!(f, "DEADLINE_EXCEEDED"),
            Status::NotFound => write!(f, "NOT_FOUND"),
            Status::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            Status::PermissionDenied => write!(f, "PERMISSION_DENIED"),
            Status::ResourceExhausted => write!(f, "RESOURCE_EXHAUSTED"),
            Status::FailedPrecondition => write!(f, "FAILED_PRECONDITION"),
            Status::Aborted => write!(f, "ABORTED"),
            Status::OutOfRange => write!(f, "OUT_OF_RANGE"),
            Status::Unimplemented => write!(f, "UNIMPLEMENTED"),
            Status::Internal => write!(f, "INTERNAL"),
            Status::Unavailable => write!(f, "UNAVAILABLE"),
            Status::DataLoss => write!(f, "DATA_LOSS"),
            Status::Unauthenticated => write!(f, "UNAUTHENTICATED"),
            Status::DatabaseError => write!(f, "DATABASE_ERROR"),
            Status::MessagingError => write!(f, "MESSAGING_ERROR"),
        }
    }
}

impl From<Status> for tonic::Code {
    fn from(reason: Status) -> Self {
        match reason {
            Status::Ok => tonic::Code::Ok,
            Status::Cancelled => tonic::Code::Cancelled,
            Status::Unknown => tonic::Code::Unknown,
            Status::InvalidArgument => tonic::Code::InvalidArgument,
            Status::DeadlineExceeded => tonic::Code::DeadlineExceeded,
            Status::NotFound => tonic::Code::NotFound,
            Status::AlreadyExists => tonic::Code::AlreadyExists,
            Status::PermissionDenied => tonic::Code::PermissionDenied,
            Status::ResourceExhausted => tonic::Code::ResourceExhausted,
            Status::FailedPrecondition => tonic::Code::FailedPrecondition,
            Status::Aborted => tonic::Code::Aborted,
            Status::OutOfRange => tonic::Code::OutOfRange,
            Status::Unimplemented => tonic::Code::Unimplemented,
            Status::Internal => tonic::Code::Internal,
            Status::Unavailable => tonic::Code::Unavailable,
            Status::DataLoss => tonic::Code::DataLoss,
            Status::Unauthenticated => tonic::Code::Unauthenticated,
            Status::DatabaseError => tonic::Code::Internal,
            Status::MessagingError => tonic::Code::Internal,
        }
    }
}
