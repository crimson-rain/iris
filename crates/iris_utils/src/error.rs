//! Error stacks for different errros which can occur in the library
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IrisUtilError {
    #[error("IO Error: {0}")]
    IO(#[from] IOError),
    #[error("Parse Error: {0}")]
    Parse(#[from] ParseError),
    #[error("File System Error: {0}")]
    FSError(#[from] FSError),
}

#[derive(Debug, Error)]
pub enum IOError {
    #[error("File Not Found")]
    NotFound,
    #[error("Permission Denied")]
    PermissionDenied,
}

#[derive(Debug, Error)]
pub enum FSError {
    #[error("No Models Found")]
    NoModelsFound,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid Syntax")]
    InvalidSyntax,
    #[error("Unexpected End of Input")]
    UnexpectedEoI,

    #[error("JSON Parse Error: {0}")]
    FailedToParseJSON(#[from] JSONError),
}

#[derive(Debug, Error)]
pub enum JSONError {
    #[error("Missing Open Braces")]
    MissingOpenBraces,
    #[error("Missing Closed Braces")]
    MissingClosedBraces,
}
