#![deny(clippy::unwrap_used, clippy::expect_used)]
use std::fmt;
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum HashassinError {
    IoError(io::Error),
    UnknownAlgorithm(String),
    EmptyInputFile,
    InconsistentPasswordLength,
    InvalidFileFormat(String),
    Utf8Error(Utf8Error),
    FromUtf8Error(FromUtf8Error),
    CoreError(String),
    CliError(String),
}

// Cool stuff, centralizing all this messages in a slingle place
impl fmt::Display for HashassinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashassinError::IoError(err) => write!(f, "I/O error {}", err),
            HashassinError::UnknownAlgorithm(algo) => write!(f, "Unknown algorithm {}", algo),
            HashassinError::EmptyInputFile => write!(f, "Empty input file"),
            HashassinError::InconsistentPasswordLength => {
                write!(f, "All passwords must be the same length")
            }
            HashassinError::InvalidFileFormat(msg) => write!(f, "Invalid file format: {}", msg),
            HashassinError::Utf8Error(err) => write!(f, "UTF-8 error: {}", err),
            HashassinError::FromUtf8Error(err) => write!(f, "UTF-8 conversion error: {}", err),
            HashassinError::CoreError(msg) => write!(f, "Core error: {}", msg),
            HashassinError::CliError(msg) => write!(f, "Cli error: {}", msg),
        }
    }
}

impl std::error::Error for HashassinError {}

impl From<io::Error> for HashassinError {
    fn from(err: io::Error) -> Self {
        HashassinError::IoError(err)
    }
}

impl From<Utf8Error> for HashassinError {
    fn from(err: Utf8Error) -> Self {
        HashassinError::Utf8Error(err)
    }
}

impl From<FromUtf8Error> for HashassinError {
    fn from(err: FromUtf8Error) -> Self {
        HashassinError::FromUtf8Error(err)
    }
}

//handling errors from hashassin_core
impl From<Box<dyn std::error::Error>> for HashassinError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        HashassinError::CoreError(err.to_string())
    }
}

pub type HashassinResult<T> = Result<T, HashassinError>;
