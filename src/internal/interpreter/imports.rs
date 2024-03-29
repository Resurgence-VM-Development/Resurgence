use super::Interpreter;
use crate::ext_func::resurgence_state::ResurgenceState;
use std::io::Error;

#[derive(Clone)]
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
        self.rust_functions.push(RustFunc {
            name: func_name,
            func: None,
            native: true,
            native_func: Some(function),
        });
    }
}
