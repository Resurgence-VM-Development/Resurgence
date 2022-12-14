use crate::internal::interpreter::imports::RustFunc;

use super::{stackframe::StackFrame, instruction::Instruction, constant::Constant};

/// All types of errors in Resurgence
#[allow(non_camel_case_types)]
pub(crate) enum ResurgenceErrorKind {
    /// Operation is invalid (ex. using a ConsantPool value in a `MOV` or `REF` instruction)
    INVALID_OPERATION,
    /// Register set to none when trying to access it (ex. `ADD`)
    MEMORY_ADDRESS_NONE,
    /// Integer overflow
    OVERFLOW,
    /// Runtime safety violated (ex. trying to register a function in a Rust API function)
    RUNTIME_SEAL_TAMPERED,
    /// Out of bounds when accessing a register
    REGISTER_OUT_OF_BOUNDS,
    /// When imports are not resolved
    MISSING_IMPORTS,
}

/// Represents a exception and stores the current state of the runtime, including callstack,
/// current instruction, and the parameters of the instruction in question
pub struct ResurgenceError {
    /// The type of error
    error_type: ResurgenceErrorKind,
    /// Error message 
    error_message: String,
    /// Context of the error
    pub context: ResurgenceContext,
}

impl ResurgenceError {
    pub(crate) fn from(error_type: ResurgenceErrorKind, error_message: &str, context: ResurgenceContext) -> ResurgenceError {
        ResurgenceError {
            error_type,
            error_message: error_message.to_string(),
            context
        }
    }
}
pub struct ResurgenceContext {
    /// Call stack at the time of exception
    call_stack: Vec<StackFrame>,
    /// Constant stack at the time of exception
    constant_stack: Vec<Constant>,
    /// All registered functions and their indexes
    rust_and_native_fns: Vec<RustFunc>,
    /// Instruction at the time of exception
    instruction: Instruction,
    /// Instruction index at time of exception
    instruction_pointer: usize,
    /// Recursion depth 
    recursion_depth: usize,
}
