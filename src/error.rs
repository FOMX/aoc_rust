use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error{ 
    #[error("io error {0}")]
    IoError(#[from] std::io::Error),
    #[error("parse error {0}")]
    ParseError(#[from] ParseError),
}


#[derive(Debug, Error)]
pub enum ParseError { // TODO: unable to do genercs
    #[error("invalid sequence: {0}")]
    InvalidSequence(&'static str),
    #[error("expected a digit found")]
    ExpectedDigit,
    #[error("unable to parse")]
    RawParseError,
    #[error("unable to parse into int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("unable to parse from Str")]
    FromStrError,
}