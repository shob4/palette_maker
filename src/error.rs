use std::{num::ParseIntError, str::ParseBoolError};

#[derive(Debug, thiserror::Error)]
pub enum PaletteError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Invalid number")]
    ParseInt(#[from] ParseIntError),

    #[error("Invalid bool")]
    ParseBool(#[from] ParseBoolError),

    #[error("Invalid palette line: {0}")]
    InvalidFormat(String),

    #[error("Unable to translate: {0}")]
    UntranslatableEncoding(String),

    #[error("Unable to display: {0}")]
    Display(String),
}
