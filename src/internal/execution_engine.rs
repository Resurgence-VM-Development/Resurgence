use std::io::Error;

/// `ExecutionEngine`: trait for implementing a Instruction interpreter
pub trait ExecutionEngine {
    /// Function that does the actual execution
    fn execute_instruction(&mut self, start_index: usize) -> Result<(), Error>;
}
