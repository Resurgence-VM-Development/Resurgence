use super::Interpreter;
use crate::{api::ext_func::resurgence_state::ResurgenceState, internal::runtime_seal::Status};
use std::io::Error;

pub struct RustFunc {
    pub name: String,
    pub func: Option<fn(&mut ResurgenceState) -> Result<(), Error>>,
    // DANGER: native MUST be set to true if using native func, and false if using func; unsafe and
    // undefined behavior WILL result if this is not adhered to. See ext_call.rs to learn why.
    pub native: bool,
    pub native_func: Option<extern "C" fn(&mut ResurgenceState) -> u8>,
}

impl Interpreter {
    /// Registers a single function to the interpreter instance
    ///
    /// `function` (`fn(&mut ResurgenceState) -> Result<(), Error>`)
    pub fn register_function(
        &mut self,
        function: fn(&mut ResurgenceState) -> Result<(), Error>,
        func_name: String,
    ) {
        // If the runtime variable is set to true, then execution has begun
        // and the runtime can no longer be trusted
        if self.seal.runtime_security_status() == Status::UNTAMPERED {
            self.seal.runtime_tampered();
        }
        self.rust_functions.push(RustFunc {
            name: func_name,
            func: Some(function),
            native: false,
            native_func: None,
        });
    }

    pub(crate) fn register_native_function(
        &mut self,
        function: extern "C" fn(&mut ResurgenceState) -> u8,
        func_name: String,
    ) {
        // If the runtime variable is set to true, then execution has begun
        // and the runtime can no longer be trusted
        if self.seal.runtime_security_status() == Status::TAMPERED {
            self.seal.runtime_tampered();
        }
        self.rust_functions.push(RustFunc {
            name: func_name,
            func: None,
            native: true,
            native_func: Some(function),
        });
    }
}
