use chrono::MappedLocalTime;

use crate::DateTime;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum ParseError {
    Json(#[from] serde_json::Error),

    Csv(#[from] csv::Error),

    #[error("Failed to parse Timestamp")]
    InvalidTimestamp,
}

pub trait ToResult<T, E> {
    fn to_result(self) -> Result<T, E>;
}

impl ToResult<DateTime, ParseError> for MappedLocalTime<DateTime> {
    fn to_result(self) -> Result<DateTime, ParseError> {
        match self {
            chrono::offset::LocalResult::Single(t) => Ok(t),
            _ => Err(ParseError::InvalidTimestamp),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    #[error("Unable to find Arbitration any data")]
    /// Pretty much a [None] variant. Happens whenever an ArbitrationInfo is not found.
    DataNotFound,

    #[error("DateTime invalidated while trying to process it")]
    /// Happens whenever a DateTime is invalidated while processing it.
    /// This should pretty much NEVER happen.
    InvalidatedDateTime,
}
