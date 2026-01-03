use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum PaletteError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Invalid number")]
    Parse(#[from] ParseIntError),

    #[error("Invalid palette line: {0}")]
    InvalidFormat(String),

    #[error("Unable to translate: {0}")]
    UntranslatableEncoding(String),
}
