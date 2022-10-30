/*!
# Resurgence FFI
Resurgence provides a foreign-function interface (FFI) to enable C code as well as other languages
to interface with a Resurgence VM.
*/

mod interpreter;
pub use interpreter::*;

mod codeholder;
pub use codeholder::*;

mod state;
pub use state::*;

use std::ffi::CString;
use std::os::raw::c_char;

/// This function frees a String (char*) that previously was given to external code. Currently,
/// this is only useful with `rvm_state_get_string`.
#[no_mangle]
pub unsafe extern "C" fn rvm_string_free(str: *mut c_char) {
    if str.is_null() {
        return;
    }

    let cstr = CString::from_raw(str);
    drop(cstr);
}
