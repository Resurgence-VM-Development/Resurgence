use std::io::Error;

use crate::ResurgenceError;

/// `ExecutionEngine`: trait for implementing a Instruction interpreter
pub trait ExecutionEngine {
    /// Function that does the actual execution
    fn execute_instruction(&mut self, start_index: usize) -> Result<(), ResurgenceError>;
    fn execute_function(&mut self, func_name: &str) -> Result<(), Error>;
}
