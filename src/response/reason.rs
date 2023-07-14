use std::fmt::{self, Debug};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ErrorReasons: Debug + Clone + Serialize + DeserializeOwned {}

/// Manages a default static list of error reasons
/// This can be overriden to a custom list of error reasons
/// when using [`crate::ErrorInfo`](crate::ErrorInfo)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorReason {
    ApiKeyInvalid,
}

impl ErrorReasons for ErrorReason {}

impl fmt::Display for ErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorReason::ApiKeyInvalid => write!(f, "API_KEY_INVALID"),
        }
    }
}
