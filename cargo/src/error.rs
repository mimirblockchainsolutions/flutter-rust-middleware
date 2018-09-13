// -------------------------------> Error
use std::{fmt, error};

#[derive(Debug)]
pub enum Error {
    Error(Box<error::Error>),
    Message(&'static str),
}

impl Error {
    pub fn message(msg: &'static str) -> Self {
        Error::Message(msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Error(err) => err.fmt(f),
            Error::Message(msg) => f.write_str(msg),
        }
    }
}

impl<T> From<T> for Error
where
    T: error::Error + 'static,
{
    fn from(err: T) -> Self {
        Error::Error(Box::new(err))
    }
}
