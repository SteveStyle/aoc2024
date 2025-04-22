use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MachineError {
    LogicError(String),
    WireError(String),
}

impl Display for MachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MachineError::LogicError(msg) => write!(f, "Logic Error: {}", msg),
            MachineError::WireError(msg) => write!(f, "Wire Error: {}", msg),
        }
    }
}

impl Error for MachineError {}

pub type Result<T> = std::result::Result<T, MachineError>;
