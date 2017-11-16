
use std::fmt;
use std::error::Error;

#[derive(Clone,Debug,PartialEq)]
pub enum SolventError {
    /// A cycle has been detected
    CycleDetected,
    NoSuchNode,
}

impl fmt::Display for SolventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => self.description().fmt(f)
        }
    }
}

impl Error for SolventError {
    fn description(&self) -> &str
    {
        match *self {
            SolventError::CycleDetected => "Cycle Detected",
            SolventError::NoSuchNode => "No Such Node",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None
        }
    }
}
