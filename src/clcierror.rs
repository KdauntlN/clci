#[derive(Debug)]
pub enum ClciError {
    InvalidFileTypeError(String),
}

impl std::error::Error for ClciError {}

impl std::fmt::Display for ClciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClciError::InvalidFileTypeError(ext) => write!(f, "Invalid file type: {ext}"),
        }
    }
}