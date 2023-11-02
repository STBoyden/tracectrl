//! The prelude for the application
//!
//! Contains:
//! - A re-export of the [`Error`] type
//! - A re-alias of the [`format!`] macro as [`fmt`]
//! - An alias called [`Result`] that uses [`Error`] as the error type

pub use crate::error::Error;
pub use std::format as fmt;

pub type Result<T> = std::result::Result<T, Error>;
