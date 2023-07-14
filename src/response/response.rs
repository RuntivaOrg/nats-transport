use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::response::{ErrorDetails, ErrorReason, MetaKeys, ResponseError, Status};

/// `StandardNatsReply` is used for NATs Request/Reply responses with
/// a standard set of ErrorReasons. Custom reasons can be implemented
/// by using the `NatsReply` struct directly.
///
/// Examples of creating success and error responses:
/// ``` rust
///     use nats_transport::response::{StandardNatsResponse, Status, ErrorReason, MetaKeys};
///     struct SampleResponse {
///         id: uuid::Uuid,
///         name: String,
///     }

///     let success_response = StandardNatsResponse::new(SampleResponse {
///         id: uuid::Uuid::new_v4(),
///         name: "test".to_string(),
///     });
///     
///     
///     let error_response: StandardNatsResponse<()> =
///     StandardNatsResponse::with_error(Status::DatabaseError)
///         .message("Unable to connect to database".to_string())
///         .with_details(
///             ErrorReason::ApiKeyInvalid,
///             "chat.persist.server".to_string(),
///         )
///         .append_metadata(
///             MetaKeys::DatabaseError,
///             "127.0.0.1".to_string(),
///         );
/// ```
pub type StandardNatsResponse<T> = NatsResponse<T, ErrorReason>;

/// `NatsReply` is used for NATs Request/Reply responses
#[derive(Debug, Serialize, Deserialize)]
pub struct NatsResponse<T, R> {
    pub error: Option<ResponseError<R>>,
    pub data: Option<T>,
}

impl<T, R> NatsResponse<T, R> {
    pub fn new(data: T) -> Self {
        Self {
            error: None,
            data: Some(data),
        }
    }

    pub fn with_error(status: Status) -> Self {
        Self {
            error: Some(ResponseError::new(status)),
            data: None,
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        debug_assert!(self.error.is_some(), "Error must be set");
        let e = self.error.as_mut().unwrap();
        e.status = status;
        e.code = status as i32;
        self
    }

    pub fn message(mut self, error_message: String) -> Self {
        debug_assert!(self.error.is_some(), "Error must be set");
        let e = self.error.as_mut().unwrap();
        e.message = error_message;
        self
    }
    pub fn with_details(mut self, reason: R, domain: String) -> Self {
        debug_assert!(self.error.is_some(), "Error must be set");
        let e = self.error.as_mut().unwrap();
        let details = ErrorDetails::new(reason, domain, HashMap::new());
        e.details.push(details);
        self
    }

    pub fn append_metadata(mut self, key: MetaKeys, value: String) -> Self {
        debug_assert!(self.error.is_some(), "Error must be set");
        let e = self.error.as_mut().unwrap();
        debug_assert!(!e.details.is_empty(), "Must call with_details first");
        let details = e.details.last_mut().unwrap();
        details.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
#[path = "./response_tests.rs"]
mod response_tests;
