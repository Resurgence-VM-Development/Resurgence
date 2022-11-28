use std::io::Error;
use std::result::Result;
pub(crate) mod execution_engine;
pub(crate) mod imports;
mod instruction;
mod utils;

use self::imports::RustFunc;
use super::super::constant::Constant;
use crate::bytecode::codereader;
use crate::internal::runtime_seal::RunTimeSeal;
use crate::objects::codeholder::CodeHolder;
use crate::objects::stackframe::StackFrame;

pub mod resolve_imports; 

/// `Interpreter`: Built-in Register Virtual Machine
pub struct Interpreter {
    /// Special register used for fast math
    accumulator: f64,
    accumulator_as_const: Constant,
    /// Holds stack frames for function calls
    call_stack: Vec<StackFrame>,
    /// Holds temporary values
    stack: Vec<Constant>,
    /// The object that holds the bytecode to iterate over
    code_holder: CodeHolder,
    /// Holds global variables
    global: Vec<Option<Constant>>,
    /// All Rust functions registered before runtime
    rust_functions: Vec<RustFunc>,
    /// Helps with validating the runtime
    seal: RunTimeSeal,
    /// Defines how many times we've recursed
    current_recursion_depth: usize,
    /// Defines the recursion limit
    max_recursion_depth: usize
}

impl Interpreter {
    /// Creates a new `Interpreter` instance using a given CodeHolder
    pub fn from(ch: CodeHolder) -> Interpreter {
        Interpreter {
            accumulator: 0.0,
            accumulator_as_const: Constant::Double(0.0),
            call_stack: Vec::new(),
            stack: Vec::new(),
            code_holder: ch,
            global: Vec::new(),
            rust_functions: Vec::new(),
            seal: RunTimeSeal::new(),
            current_recursion_depth: 0,
            max_recursion_depth: 1000
        }
    }

    /// Reads a file at a given path, parses it, and creates an [`Interpreter`] instance.
    ///
    /// This is a convenience wrapper for [`crate::api::codereader::read_bytecode_file`] and
    /// behaves the same way.
    pub fn from_file(path: &str) -> Result<Interpreter, Error> {
        Ok(Self::from(codereader::read_bytecode_file(path)?))
    }

    /// Modifies the max recursion depth
    ///
    /// new_depth (`usize`): The new max depth
    pub fn set_max_depth(&mut self, new_depth: usize) {
        self.max_recursion_depth = new_depth;
    }

    /// Returns the call stack from the runtime - Invalidates the runtime if called
    pub(crate) fn yoink_call_stack(&self) -> Vec<StackFrame> {
        self.seal.runtime_tampered();
        self.call_stack
    }
    
    /// Returns the constant stack - Invalidates the runtime if called
    pub(crate) fn yoink_stack(&self) -> Vec<Constant> {
        self.seal.runtime_tampered();
        self.stack
    }
   
    /// Returns the table of Rust and native functions - Invalidates the runtime if called 
    pub(crate) fn yoink_rust_c(&self) -> Vec<RustFunc> {
        self.seal.runtime_tampered();
        self.rust_functions
    }
}
