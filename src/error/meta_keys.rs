use std::{fmt, hash::Hash};

use serde::{Deserialize, Serialize};

pub trait ErrorMetaKeys {}

/// Manages a default static keys for metadata
/// This can be overriden to a custom list of metakeys
/// when using [`crate::ErrorInfo`](crate::ErrorInfo)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MetaKeys {
    Requestor,
    Request,
    Service,
    DatabaseError,
    OtherError,
}

impl ErrorMetaKeys for MetaKeys {}

impl fmt::Display for MetaKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetaKeys::Request => write!(f, "request"),
            MetaKeys::Requestor => write!(f, "requestor"),
            MetaKeys::Service => write!(f, "service"),
            MetaKeys::DatabaseError => write!(f, "DatabaseError"),
            MetaKeys::OtherError => write!(f, "OtherError"),
        }
    }
}
