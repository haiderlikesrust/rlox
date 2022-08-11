use std::{num::ParseIntError, str::ParseBoolError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Runtime {
    #[error("Not an integer")]
    NotInt(#[from] ParseIntError),
    #[error("Not a string")]
    NotString,
    #[error("Operation is invalid")]
    InvalidOperation,
    #[error("Not a boolean")]
    NotBool(#[from] ParseBoolError),
}