use std::io::{Error, ErrorKind};

use crate::{api::ext_func::resurgence_state::ResurgenceState, Interpreter};

impl Interpreter {
    pub(crate) fn ext_call(&self, index: u64) -> Result<(), Error> {
        let real_id = &self.code_holder.byte_to_interal[index as usize];
        let mut state = ResurgenceState::new(&self.stack);

        let function = &self.rust_functions[*real_id as usize];

        if function.native {
            // function.native guarantees this will succeed, so it should be safe
            let func = unsafe { function.native_func.unwrap_unchecked() };
            let ec = (func)(&mut state);
            if ec != 0 {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "Native function \"{}\" returned nonzero status code {}",
                        function.name, ec
                    ),
                ));
            }

            return Ok(());
        } else {
            // function.native guarantees this will succeed, so it should be safe
            let func = unsafe { function.func.unwrap_unchecked() };
            return (func)(&mut state);
        }
    }
}
