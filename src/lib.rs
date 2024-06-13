use error::Error;

pub mod error;
pub mod model;

pub type Result<T> = std::result::Result<T, Error>;
