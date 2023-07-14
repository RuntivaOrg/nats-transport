use std::collections::HashMap;

use chat_proto::chat as proto;
use serde::{Deserialize, Serialize};

use crate::response::{MetaKeys, Status};

/// Structured error response between services.
/// This is based on GCP Cloud API Error best practices:
/// https://cloud.google.com/apis/design/errors#error_model
///
/// Example:
///
/// ```json
///   {
///     "error": {
///       "code": 400,
///       "message": "API key not valid. Please pass a valid API key.",
///       "status": "INVALID_ARGUMENT",
///       "details": [
///         {
///           "@type": "type.googleapis.com/google.rpc.ErrorInfo",
///           "reason": "API_KEY_INVALID",
///           "domain": "googleapis.com",
///           "metadata": {
///             "service": "translate.googleapis.com"
///           }
///         }
///       ]
///     }
///   }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError<R> {
    pub code: i32,

    pub message: String,

    pub status: Status,

    pub details: Vec<ErrorDetails<R>>,
}

impl<R> ResponseError<R> {
    #[must_use]
    pub fn new(status: Status) -> Self {
        Self {
            status,
            code: status as i32,
            message: String::new(),
            details: vec![],
        }
    }
}

impl<R> From<ResponseError<R>> for proto::ErrorReply
where
    R: ToString,
{
    fn from(val: ResponseError<R>) -> proto::ErrorReply {
        let mut details = Vec::<proto::ErrorDetails>::new();
        for detail in val.details {
            details.push(detail.into());
        }

        proto::ErrorReply {
            code: val.code,
            message: val.message,
            status: val.status as i32,
            details,
        }
    }
}

/// Describes the cause of the error with structured details.
/// This is based on GCP Cloud API Error best practices:
/// https://cloud.google.com/apis/design/errors#error_model
///
/// Example:
///
/// ```json
///     { "reason": "INVALID_REQUEST"
///       "domain": "chat.persistance.server"
///       "metadata": {
///         "resource": "topic/12345678-1234-1234-123456781234",
///         "service": "chat.persistance.server"
///       }
///     }
///  ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails<R> {
    // The reason of the error. This is a constant value that identifies the
    // proximate cause of the error. Error reasons are unique within a particular
    // domain of errors. This should be at most 63 characters and match a
    // regular expression of `[A-Z][A-Z0-9_]+[A-Z0-9]`, which represents
    // UPPER_SNAKE_CASE.
    //#[serde(deserialize_with = "R::deserialize")]
    pub reason: R,

    // The logical grouping to which the "reason" belongs. The error domain
    // is typically the registered service name of the tool or product that
    // generates the error. Example: "pubsub.googleapis.com". If the error is
    // generated by some common infrastructure, the error domain must be a
    // globally unique value that identifies the infrastructure. For Google API
    // infrastructure, the error domain is "googleapis.com".
    pub domain: String,

    // Additional structured details about this error.
    //
    // Keys should match /[a-zA-Z0-9-_]/ and be limited to 64 characters in
    // length. When identifying the current value of an exceeded limit, the units
    // should be contained in the key, not the value.  For example, rather than
    // {"instanceLimit": "100/request"}, should be returned as,
    // {"instanceLimitPerRequest": "100"}, if the client exceeds the number of
    // instances that can be created in a single (batch) request.
    pub metadata: HashMap<MetaKeys, String>,
}

impl<R> ErrorDetails<R> {
    pub fn new(reason: R, domain: String, metadata: HashMap<MetaKeys, String>) -> Self {
        Self {
            reason,
            domain,
            metadata,
        }
    }
}

impl<R> Into<proto::ErrorDetails> for ErrorDetails<R>
where
    R: ToString,
{
    fn into(self) -> proto::ErrorDetails {
        let mut metadata = Vec::<proto::MetaData>::new();
        for (key, value) in self.metadata {
            let entry = proto::MetaData {
                key: key.to_string(),
                value: value.to_string(),
            };
            metadata.push(entry);
        }

        proto::ErrorDetails {
            reason: self.reason.to_string(),
            domain: self.domain,
            metadata,
        }
    }
}
