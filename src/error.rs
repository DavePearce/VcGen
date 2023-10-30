use crate::{MachineError};

/// A minimal implementation of `MachineError` which contains only two
/// error kinds.
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum MinimalMachineError {
    /// Indicates an attempt was made to read an instruction that does
    /// not exist (e.g. is beyond the end of the code section).
    InvalidPC,
    /// Indicates an attempt was made to pop an item from an empty
    /// stack.
    StackUnderflow
}

impl MachineError for MinimalMachineError {
    fn invalid_pc() -> Self {
        MinimalMachineError::InvalidPC
    }
    
    fn stack_underflow() -> Self {
        MinimalMachineError::StackUnderflow
    }
    
}
