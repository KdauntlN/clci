#[derive(Debug)]
pub enum ClciError {
    InvalidFileTypeError(String),
    MalformedSectionHeader(usize, String),
    MissingAssignmentOperator(usize, String),
}

impl std::error::Error for ClciError {}

impl std::fmt::Display for ClciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClciError::InvalidFileTypeError(ext) => write!(f, "Invalid file type \"{ext}\""),
            ClciError::MalformedSectionHeader(i, line) => {
                write!(f, "Malformed section header (line {i}): \"{line}\"")
            }
            ClciError::MissingAssignmentOperator(i, line, ) => {
                write!(f, "Missing assignment operator (line {i}): \"{line}\"")
            }
        }
    }
}