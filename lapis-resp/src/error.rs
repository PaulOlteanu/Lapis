use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEOF,
    Other,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedEOF => f.write_str("unexpected eof"),
            Error::Other => f.write_str("unknown error"),
        }
    }
}

impl std::error::Error for Error {}
