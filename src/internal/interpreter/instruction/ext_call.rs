use crate::{Interpreter, api::ext_func::resurgence_state::ResurgenceState};


impl Interpreter {
    pub fn ext_call(&self, index: u64) {
        let function = &self.rust_functions[index as usize];
        let state = ResurgenceState::from(self.stack.clone());
        let function_res = (function.func)(state);
        if let Err(err_msg) = function_res {
            panic!("{}", err_msg);
        }
    }
}