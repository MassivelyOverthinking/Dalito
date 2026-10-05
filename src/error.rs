//====================================================================================
// IMPORTS & MODULES
//====================================================================================

use std::fmt;
use std::error::Error;

//====================================================================================
// CUSTON ERRORS
//====================================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DalitoErrorType {
    InvalidFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DalitoError {          // Simple Error with custom messaging
    // Input validation errors for public API boundaries.
    InvalidFormat {
        format: &'static str,
    },
}

impl DalitoError {
    #[inline]
    pub fn kind(&self) -> DalitoErrorType {
        match self {
            DalitoError::InvalidFormat { .. } => DalitoErrorType::InvalidFormat,
        }
    }

    #[inline]
    pub fn invalid_format(format: &'static str) -> Self {
        DalitoError::InvalidFormat { format }
    }
}


impl fmt::Display for DalitoError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat { format} => 
            write!(f, "Invalid data format {}", format),
        }
    }
}

// Error Trait for DalitoError
impl Error for DalitoError {}