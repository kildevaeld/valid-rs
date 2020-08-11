use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Required,
    MinLen(usize),
    MaxLen(usize),
    Other(Box<dyn StdError>),
    Multi(Vec<Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl StdError for Error {}
