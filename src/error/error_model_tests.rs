#[cfg(test)]
mod error_model_tests {

    pub use crate::error::{ErrorModel, MetaKeys, Status, ToErrorModel};

    use super::MySampleError;

    #[test]
    fn test_error_model() {
        let err = MySampleError {
            msg: "No chat title provided.".to_string(),
            reason: super::SampleErrorReason::ChatTitleEmpty,
            source: super::SampleError::InvalidArgument(
                "Missing Argument: No chat title provided.".to_string(),
            ),
        };

        let model = err.to_error_model(
            Some(1234567890),
            Some("chat.chatgroup.command.create".to_string()),
        );

        //panic!("{:?}", model.details.first().unwrap().metadata.len(), 3);
        assert_eq!(model.code, 400);
        assert_eq!(model.status, Status::InvalidArgument);
        assert_eq!(model.message, "No chat title provided.".to_string());
        assert_eq!(model.details.len(), 1);

        assert_eq!(
            model.details.first().unwrap().reason,
            super::SampleErrorReason::ChatTitleEmpty
        );
        assert_eq!(
            model.details.first().unwrap().domain,
            "runtiva.com".to_string()
        );
        assert_eq!(
            model
                .details
                .first()
                .unwrap()
                .metadata
                .get(&MetaKeys::Requestor)
                .unwrap(),
            &1234567890.to_string()
        );
        assert_eq!(
            model
                .details
                .first()
                .unwrap()
                .metadata
                .get(&MetaKeys::Request)
                .unwrap(),
            &"chat.chatgroup.command.create".to_string()
        );
        assert_eq!(
            model
                .details
                .first()
                .unwrap()
                .metadata
                .get(&MetaKeys::Service)
                .unwrap(),
            &"chat-persist.runtiva.com".to_string()
        );
    }
}

use std::{fmt, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::error::{ErrorReasons, MetaKeys, Status};

use super::{ErrorModel, ToErrorModel};

#[derive(thiserror::Error, Debug)]
pub struct MySampleError {
    msg: String,
    reason: SampleErrorReason,
    #[source]
    source: SampleError,
}

impl Display for MySampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SampleError {
    #[error("{0}")]
    InvalidArgument(String),

    #[allow(dead_code)]
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ToErrorModel<SampleErrorReason> for MySampleError {
    fn to_error_model(
        &self,
        requestor: Option<i64>,
        request: Option<String>,
    ) -> ErrorModel<SampleErrorReason> {
        let mut model = ErrorModel::new(self.status(), self.error_code(), self.msg());

        model = model
            .with_details(self.reason, "runtiva.com".to_string())
            .append_metadata(MetaKeys::Service, "chat-persist.runtiva.com".to_string());

        if let Some(request) = request {
            model = model.append_metadata(MetaKeys::Request, request);
        }

        if let Some(requestor) = requestor {
            model = model.append_metadata(MetaKeys::Requestor, requestor.to_string());
        }

        model
    }

    fn msg(&self) -> String {
        self.msg.to_string()
    }

    fn error_code(&self) -> i32 {
        match &self.source {
            SampleError::InvalidArgument(_) => 400,
            SampleError::InternalError { .. } => 500,
        }
    }

    fn status(&self) -> Status {
        match &self.source {
            SampleError::InvalidArgument(_) => Status::InvalidArgument,
            SampleError::InternalError { .. } => Status::Internal,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum SampleErrorReason {
    ChatTitleEmpty,
    ChatAboutTooLong,
}

impl ErrorReasons for SampleErrorReason {}

impl Display for SampleErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SampleErrorReason::ChatTitleEmpty => write!(f, "CHAT_TITLE_EMPTY"),
            SampleErrorReason::ChatAboutTooLong => write!(f, "CHAT_ABOUT_TOO_LONG"),
        }
    }
}
