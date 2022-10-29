use std::io::Error;
use std::result::Result;

pub(crate) mod execution_engine;
mod instruction;
pub(crate) mod rust_func;
mod utils;

use self::rust_func::RustFunc;
use super::super::constant::Constant;
use crate::api::codereader;
use crate::api::ext_func::resurgence_state::ResurgenceState;
use crate::objects::codeholder::CodeHolder;
use crate::objects::stackframe::StackFrame;

pub(crate) mod resolve_imports; // Resurgence already handles this at runtime, so it's only public to the crate

/// `Interpreter`: Built-in Register Virtual Machine
pub struct Interpreter {
    /// Special register used for fast math
    accumulator: f64,
    accumulator_as_const: Constant,

    /// Holds stack frames for function calls
    call_stack: Vec<StackFrame>,

    /// Holds temporary values
    stack: Vec<Constant>,

    code_holder: CodeHolder,

    /// Holds global variables
    global: Vec<Option<Constant>>,

    // All Rust functions registered before runtime
    rust_functions: Vec<RustFunc>,
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
        }
    }

    /// Reads a file at a given path, parses it, and creates an [`Interpreter`] instance.
    ///
    /// This is a convenience wrapper for [`crate::api::codereader::read_bytecode_file`] and
    /// behaves the same way.
    pub fn from_file(path: &str) -> Result<Interpreter, Error> {
        Ok(Self::from(codereader::read_bytecode_file(path)?))
    }

    /// Registers a single function to the interpreter instance
    ///
    /// `function` (`)
    pub fn register_function(
        &mut self,
        function: fn(&mut ResurgenceState) -> Result<(), Error>,
        func_name: String,
    ) {
        self.rust_functions.push(RustFunc {
            name: func_name,
            func: function,
        });
    }
}
