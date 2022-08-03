mod utils;
mod instruction;
pub(crate) mod execution_engine;

use crate::objects::stackframe::StackFrame;
use super::super::constant::Constant;

/// `Interpreter`: Built-in Register Virtual Machine
pub struct Interpreter {
    /// Special register used for fast math
    accumulator: f64,

    /// Holds stack frames for function calls
    call_stack: Vec<StackFrame>,

    /// Holds temporary values
    stack: Vec<Constant>,

    /// Holds global variables
    global: Vec<Option<Constant>>
}

impl Interpreter {
    /// Creates a new `Interpreter` instance
    pub fn new() -> Interpreter {
        Interpreter {
            accumulator: 0.0,
            call_stack: Vec::new(),
            stack: Vec::new(),
            global: Vec::new()
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
