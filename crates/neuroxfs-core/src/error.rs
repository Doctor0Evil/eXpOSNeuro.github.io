use std::io;

#[derive(Debug)]
pub enum FsError {
    Io(io::Error),
    GuardError(String),
    ModeError(String),
    PolicyError(String),
}

impl std::fmt::Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsError::Io(e) => write!(f, "IO error: {}", e),
            FsError::GuardError(m) => write!(f, "Guard error: {}", m),
            FsError::ModeError(m) => write!(f, "Mode error: {}", m),
            FsError::PolicyError(m) => write!(f, "Policy error: {}", m),
        }
    }
}

impl std::error::Error for FsError {}
