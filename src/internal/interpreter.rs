use smartstring::alias::String;

#[allow(non_snake_case)]
pub struct Interpreter {
    pub args: Vec<String>,
    
    accumulator: f64,
    func_ret: Constant,
    func_args: Vec<Constant>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            args: Vec::new()
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<String>> for Interpreter {
    fn from(passed_args: Vec<String>) -> Self {
        Interpreter {
            args: passed_args
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