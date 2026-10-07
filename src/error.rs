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
    InterruptedTransformation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DalitoError {          
    // Input Formatting error ==> Data file formatting is not supported.
    InvalidFormat {
        message: &'static str,
    },

    // Transformation error ==> A lineage transformation is actively being recorded and cannot be interrupted.
    InterruptedTransformation {
        message: &'static str,
    }
}

impl DalitoError {
    #[inline]
    pub fn kind(&self) -> DalitoErrorType {
        match self {
            DalitoError::InvalidFormat { .. } => DalitoErrorType::InvalidFormat,
            DalitoError::InterruptedTransformation { .. } => DalitoErrorType::InterruptedTransformation,
        }
    }

    #[inline]
    pub fn invalid_format(message: &'static str) -> Self {
        DalitoError::InvalidFormat { message }
    }

    #[inline]
    pub fn interrupted_transformation(message: &'static str) -> Self {
        DalitoError::InterruptedTransformation { message }
    }
}


impl fmt::Display for DalitoError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat { message} => 
            write!(f, "Invalid data format {}", message),
            Self::InterruptedTransformation { message } =>
            write!(f, "Active transformation cannot be interrupted {}", message),
        }
    }
}

// Error Trait for DalitoError
impl Error for DalitoError {}