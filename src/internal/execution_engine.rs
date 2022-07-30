use crate::objects::codeholder::CodeHolder;

/// `ExecutionEngine`: trait for implementing a Instruction interpreter
pub trait ExecutionEngine {

    /// Function that does the actual execution
    fn execute_Instruction(&mut self, code_holder: &CodeHolder, start_index: usize);
}