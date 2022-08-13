use std::io::Error;

use crate::{Interpreter, api::ext_func::resurgence_state::ResurgenceState};


impl Interpreter {
    pub fn ext_call(&self, index: u64) -> Result<(), Error> {
        let real_id = &self.byte_to_interal[index as usize];
        let function = &self.rust_functions[*real_id as usize];
        let state = ResurgenceState::from(self.stack.clone());
        (function.func)(state)
    }
}