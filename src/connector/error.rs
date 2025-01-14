//! Error module for `connector`. Holds code regarding `ConnectionError` implementations.

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Contains error type information for `ConnectionError`.
pub enum ConnectorErrorKind {
    ConnectionError,
    Unknown,
}

impl Display for ConnectorErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectorErrorKind::ConnectionError => {
                write!(f, "ConnectionError")
            }
            ConnectorErrorKind::Unknown => {
                write!(f, "Unknown")
            }
        }
    }
}

/// Contains information regarding errors with `Connector` objects.
pub struct ConnectorError {
    kind: ConnectorErrorKind,
    message: &'static str,
}

impl ConnectorError {
    /// Default formatter for ConnectorError.
    fn fmt_default(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.message, self.kind)
    }
}

impl Debug for ConnectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_default(f)
    }
}

impl Display for ConnectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_default(f)
    }
}

impl Error for ConnectorError {}
