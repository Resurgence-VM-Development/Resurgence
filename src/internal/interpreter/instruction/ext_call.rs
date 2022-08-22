use std::io::Error;

use crate::{api::ext_func::resurgence_state::ResurgenceState, Interpreter};

impl Interpreter {
    pub fn ext_call(&self, index: u64) -> Result<(), Error> {
        let real_id = &self.byte_to_interal[index as usize];
        let function = &self.rust_functions[*real_id as usize];
        let mut state = ResurgenceState::from(self.stack.clone());
        (function.func)(&mut state)
    }
}
