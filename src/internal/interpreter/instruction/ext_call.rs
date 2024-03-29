use crate::{ext_func::resurgence_state::ResurgenceState, Interpreter, ResurgenceError, objects::resurgence_error::ResurgenceErrorKind, create_new_trace};

impl Interpreter {
    pub(crate) fn ext_call(&mut self, index: u64) -> Result<(), ResurgenceError> {
        let real_id = &self.code_holder.byte_to_interal[index as usize];
        let mut state = ResurgenceState::new(&mut self.stack);

        let function = &self.rust_functions[*real_id as usize];

        if function.native {
            // function.native guarantees this will succeed, so it should be safe
            let func = unsafe { function.native_func.unwrap_unchecked() };
            let ec = (func)(&mut state);
            if ec != 0 {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::FUNCTION_RETURN_ERROR, &format!("Native function \"{}\" returned nonzero status code {}", function.name, ec));
                create_new_trace!(err);
            }

            return Ok(());
        } else {
            // function.native guarantees this will succeed, so it should be safe
            let func = unsafe { function.func.unwrap_unchecked() };
            let res = (func)(&mut state);
            if let Err(err) = res {
                let mut err = ResurgenceError::from(ResurgenceErrorKind::FUNCTION_RETURN_ERROR, &err.to_string());
                create_new_trace!(err);
                return Err(err);
            }
            return Ok(());
        }
    }
}
