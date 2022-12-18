use std::fmt;

use crate::internal::interpreter::imports::RustFunc;

use super::{stackframe::StackFrame, instruction::Instruction, constant::Constant};

#[macro_use]
mod macros {
    /// Creates a new trace in the format that Resurgence uses
    #[macro_export] macro_rules! create_new_trace {
        ($err:expr) => {
            $err.add_trace(&format!("In file {} on line {}, collum {}", file!(), line!(), column!())); 
        };
    }   
}
/// All types of errors in Resurgence
#[allow(non_camel_case_types)]
pub(crate) enum ResurgenceErrorKind {
    /// Operation is invalid (ex. using a ConsantPool value in a `MOV` or `REF` instruction)
    INVALID_OPERATION,
    /// Register set to none when trying to access it (ex. `ADD`)
    MEMORY_ADDRESS_NONE,
    /// Integer overflow
    OVERFLOW,
    /// Out of bounds when accessing a register
    REGISTER_OUT_OF_BOUNDS,
    /// When imports are not resolved
    MISSING_IMPORTS,
    /// When a function returns an error
    FUNCTION_RETURN_ERROR,
    /// When the programmer tries to call a function that doesn't exist
    FUNCTION_DOES_NOT_EXIST,

    /// When something is so messed up that you don't have the words to describe it
    I_GOOFED_UP,
}

/// Represents a exception and stores the current state of the runtime, including callstack,
/// current instruction, and the parameters of the instruction in question
pub struct ResurgenceError {
    /// The type of error
    error_type: ResurgenceErrorKind,
    /// Error message 
    error_message: String,
    /// Context of the error
    pub context: Option<ResurgenceContext>,
    /// Traceback
    trace_back: Vec<String>
}

impl ResurgenceError {
    /// Creates a new `ResurgenceError` object
    ///
    /// error_type (`ResurgenceErrorKind`): The type of error
    /// error_message (`&str`): The message to output
    /// context (`ResurgenceContext`): The state of the interpreter at the time of the error
    pub(crate) fn from(error_type: ResurgenceErrorKind, error_message: &str) -> ResurgenceError {
        ResurgenceError {
            error_type,
            error_message: error_message.to_string(),
            context: Option::None,
            trace_back: Vec::new()
        }
    }
    
    /// Adds a trace to the traceback
    ///
    /// trace (`&str`): The new trace to add
    pub(crate) fn add_trace(&mut self, trace: &str) {
        self.trace_back.push(trace.to_string());
    }
}

impl fmt::Debug for ResurgenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match *&self.error_type {
            ResurgenceErrorKind::INVALID_OPERATION => "INVALID_OPERATION",
            ResurgenceErrorKind::MEMORY_ADDRESS_NONE => "MEMORY_ADDRESS_NONE",
            ResurgenceErrorKind::OVERFLOW => "OVERFLOW",
            ResurgenceErrorKind::REGISTER_OUT_OF_BOUNDS => "REGISTER_OUT_OF_BOUNDS",
            ResurgenceErrorKind::MISSING_IMPORTS => "MISSING_IMPORTS",
            ResurgenceErrorKind::FUNCTION_RETURN_ERROR => "FUNCTION_RETURN_ERROR",
            ResurgenceErrorKind::FUNCTION_DOES_NOT_EXIST => "FUNCTION_DOES_NOT_EXIST",
            ResurgenceErrorKind::I_GOOFED_UP => "I_GOOFED_UP"

        };
        f.debug_struct("ResurgenceError")
            .field("Error Type", &type_str)
            .field("Error Message", &self.error_message)
            .finish()
    }
}

/// Represents the interpreter state at the time of creation
pub struct ResurgenceContext {
    /// Call stack at the time of exception
    pub(crate) call_stack: Vec<StackFrame>,
    /// Constant stack at the time of exception
    pub(crate) constant_stack: Vec<Constant>,
    /// All registered functions and their indexes
    pub(crate) rust_and_native_fns: Vec<RustFunc>,
    /// Instruction at the time of exception
    pub(crate) instruction: Instruction,
    /// Instruction index at time of exception
    pub(crate) instruction_pointer: usize,
    /// Recursion depth 
    pub(crate) recursion_depth: usize,
}
