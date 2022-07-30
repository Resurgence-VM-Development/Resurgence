pub(crate) mod execution_engine;

use crate::objects::stackframe::StackFrame;
use super::super::constant::Constant;
use smartstring::alias::String;

/// `Interpreter`: Built-in Register Virtual Machine
pub struct Interpreter {
    /// Arguments used by the instance
    pub args: Vec<String>,

    /// Special register used for fast math
    accumulator: f64,

    /// Holds stack frames for function calls
    call_stack: Vec<StackFrame>,

    /// Holds temporary values
    stack: Vec<Constant>,

    /// Holds global variables
    global: Vec<Constant>
}

impl Interpreter {
    /// Creates a new `Interpreter` instance
    pub fn new() -> Interpreter {
        Interpreter {
            args: Vec::new(),
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

impl From<Vec<String>> for Interpreter {
    /// Creates a new `Interpreter` using the passed args
    /// 
    /// `passed_args` (`Vec<String>`): Arguments you want to pass to the Interpreter instance
    fn from(passed_args: Vec<String>) -> Self {
        Interpreter {
            args: passed_args,
            accumulator: 0.0,
            call_stack: Vec::new(),
            stack: Vec::new(),
            global: Vec::new()
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use smartstring::alias::String;

    #[test]
    fn create_instance() {
        let test_args: Vec<String> = vec![String::from("--debug")];
        let interpreter_instance = Interpreter::from(test_args.clone());
        assert_eq!(interpreter_instance.args, test_args);
    }
}
