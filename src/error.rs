use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum SolventError {
    /// A cycle has been detected
    CycleDetected(String),
    NoSuchNode,
}

impl fmt::Display for SolventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SolventError::CycleDetected(ref s) => write!(f, "Cycle Detected: {}", s),
            SolventError::NoSuchNode => write!(f, "No Such Node"),
        }
    }
}

impl Error for SolventError {}
