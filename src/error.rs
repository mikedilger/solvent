
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
            _ => self.to_string().fmt(f)
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
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            _ => None
        }
    }
}
