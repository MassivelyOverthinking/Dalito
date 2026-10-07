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
    NoActiveTransformation,
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
    },

    // Transformation error ==> No current active lineage.
    NoActiveTransformation {
        message: &'static str,
    }
}

impl DalitoError {
    #[inline]
    pub fn kind(&self) -> DalitoErrorType {
        match self {
            DalitoError::InvalidFormat { .. } => DalitoErrorType::InvalidFormat,
            DalitoError::InterruptedTransformation { .. } => DalitoErrorType::InterruptedTransformation,
            DalitoError::NoActiveTransformation { .. } => DalitoErrorType::NoActiveTransformation,
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

    #[inline]
    pub fn no_active_transformation(message: &'static str) -> Self {
        DalitoError::InterruptedTransformation { message }
    }
}


impl fmt::Display for DalitoError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat { message} => 
            write!(f, "Invalid data format {}", message),
            Self::InterruptedTransformation { message } =>
            write!(f, "Inturrpted transformation error {}", message),
            Self::NoActiveTransformation { message } =>
            write!(f, "No active transformation: {}", message),
        }
    }
}

// Error Trait for DalitoError
impl Error for DalitoError {}