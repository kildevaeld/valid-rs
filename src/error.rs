use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Required,
    MinLen(usize),
    MaxLen(usize),
    Other(Box<dyn StdError>),
    Multi(Vec<Error>),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Required => write!(f, "required"),
            Error::MinLen(m) => write!(f, "min len {}", m),
            Error::MaxLen(m) => write!(f, "max len {}", m),
            Error::Other(m) => write!(f, "{}", m),
            Error::Multi(m) => {
                let out = m
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}", out)
            }
            Error::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl StdError for Error {}
