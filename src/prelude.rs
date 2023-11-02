pub use crate::error::Error;
pub use std::format as fmt;

pub type Result<T> = std::result::Result<T, Error>;
