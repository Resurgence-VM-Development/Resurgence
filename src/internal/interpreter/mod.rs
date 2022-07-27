pub mod execution_engine;

use crate::objects::{bytecode::ByteCode, stackframe::StackFrame};
use super::super::constant::Constant;
use smartstring::alias::String;

/// `Interpreter`: Built-in Register Virtual Machine\
/// 
/// `args` (`Vec<String>`): Arguments used by the instance
pub struct Interpreter {
    pub args: Vec<String>,
    
    accumulator: f64,
    func_ret: Constant,
    func_args: Vec<Constant>,

    bytecode: Vec<ByteCode>,

    stack: Vec<StackFrame>,
}

impl Interpreter {
    /// Creates a new `Interpreter` instance
    pub fn new() -> Interpreter {
        Interpreter {
            args: Vec::new(),
            accumulator: 0.0,
            func_ret: Constant::Int(0),
            func_args: Vec::new(),
            bytecode: Vec::new(),
            stack: Vec::new(),
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
            args: Vec::new(),
            accumulator: 0.0,
            func_ret: Constant::Int(0),
            func_args: Vec::new(),
            bytecode: Vec::new(),
            stack: Vec::new(),
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