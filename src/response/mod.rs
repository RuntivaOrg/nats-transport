use serde::{Deserialize, Serialize};

use crate::error::{ErrorModel, ErrorReason, ToErrorModel};

/// `StandardNatsReply` is used for NATs Request/Reply responses with
/// a standard set of ErrorReasons. Custom reasons can be implemented
/// by using the `NatsReply` struct directly.
///
/// Examples of creating success and error responses:
/// ``` rust
///     use std::{fmt, fmt::Display};
///     use serde::{Deserialize, Serialize};
///     use nats_transport::error::{ErrorReasons, MetaKeys, Status, ErrorModel, ToErrorModel};
///     use nats_transport::response::{StandardNatsResponse, NatsResponse};
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
///     let error_response: NatsResponse<(), SampleErrorReason> =
///     NatsResponse::with_error(
///         SampleError::InvalidArgument("No chat title provided.".to_string()),
///         SampleErrorReason::ChatTitleEmpty,
///         Some(7037539637825798),
///         Some("chat.chatgroup.command.create".to_string()));
///
///
///     #[derive(Debug, thiserror::Error)]
///     pub enum SampleError {
///         #[error("{0}")]
///         InvalidArgument(String),
///
///         #[allow(dead_code)]
///         #[error("Internal error: {0}")]
///         InternalError(String),
///     }
///
///     impl ToErrorModel<SampleErrorReason> for SampleError {
///         fn to_error_model(
///             &self,
///             requestor: Option<i64>,
///             request: Option<String>,
///             reason: SampleErrorReason,
///         ) -> ErrorModel<SampleErrorReason> {
///             let mut model = ErrorModel::new(self.status(), self.error_code(), self.to_string());
///
///             model = model
///                 .with_details(reason, "runtiva.com".to_string())
///                 .append_metadata(MetaKeys::Service, "chat-persist.runtiva.com".to_string());
///
///             if let Some(request) = request {
///                 model = model.append_metadata(MetaKeys::Request, request);
///             }
///
///             if let Some(requestor) = requestor {
///                 model = model.append_metadata(MetaKeys::Requestor, requestor.to_string());
///             }
///
///             model
///         }
///
///         fn error_code(&self) -> i32 {
///             match self {
///                 SampleError::InvalidArgument(_) => 400,
///                 SampleError::InternalError { .. } => 500,
///             }
///         }
///
///         fn status(&self) -> Status {
///             match self {
///                 SampleError::InvalidArgument(_) => Status::InvalidArgument,
///                 SampleError::InternalError { .. } => Status::Internal,
///             }
///         }
///     }
///     
///     #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
///     pub enum SampleErrorReason {
///         ChatTitleEmpty,
///         ChatAboutTooLong,
///     }
///     
///     impl ErrorReasons for SampleErrorReason {}
///     
///     impl Display for SampleErrorReason {
///         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///             match self {
///                 SampleErrorReason::ChatTitleEmpty => write!(f, "CHAT_TITLE_EMPTY"),
///                 SampleErrorReason::ChatAboutTooLong => write!(f, "CHAT_ABOUT_TOO_LONG"),
///             }
///         }
///     }
/// ```
pub type StandardNatsResponse<T> = NatsResponse<T, ErrorReason>;

/// `NatsReply` is used for NATs Request/Reply responses
#[derive(Debug, Serialize, Deserialize)]
pub struct NatsResponse<T, R> {
    pub error: Option<ErrorModel<R>>,
    pub data: Option<T>,
}

impl<T, R> NatsResponse<T, R> {
    pub fn new(data: T) -> Self {
        Self {
            error: None,
            data: Some(data),
        }
    }

    pub fn with_error(
        err: impl ToErrorModel<R>,
        reason: R,
        requestor: Option<i64>,
        request: Option<String>,
    ) -> Self {
        Self {
            error: Some(err.to_error_model(requestor, request, reason)),
            data: None,
        }
    }
}
