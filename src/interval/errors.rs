use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidIntervalError;

impl fmt::Display for InvalidIntervalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid interval!")
    }
}

impl error::Error for InvalidIntervalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
