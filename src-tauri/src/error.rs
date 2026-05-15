use std::fmt;

#[derive(Debug)]
pub enum AppError {
    CommandFailed { cmd: String, stderr: String },
    ParseError { context: String, detail: String },
    ManagerNotFound(String),
    ManagerNotAvailable(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::CommandFailed { cmd, stderr } => {
                write!(f, "Command '{}' failed: {}", cmd, stderr)
            }
            AppError::ParseError { context, detail } => {
                write!(f, "Parse error in {}: {}", context, detail)
            }
            AppError::ManagerNotFound(name) => {
                write!(f, "Package manager '{}' not found", name)
            }
            AppError::ManagerNotAvailable(name) => {
                write!(f, "Package manager '{}' is not available on this system", name)
            }
        }
    }
}

impl std::error::Error for AppError {}

impl From<AppError> for String {
    fn from(err: AppError) -> String {
        err.to_string()
    }
}
