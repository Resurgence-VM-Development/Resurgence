use crate::objects::codeholder::CodeHolder;

/// `ExecutionEngine`: trait for implementing a bytecode interpreter
pub trait ExecutionEngine {
    fn execute_bytecode(&mut self, code_holder: &CodeHolder, start_index: usize);
}