use crate::api::codereader;
use crate::CodeHolder;
use std::boxed::Box;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

/// Creates an empty CodeHolder instance.
#[no_mangle]
pub unsafe extern "C" fn rvm_codeholder_new() -> *mut CodeHolder {
    return Box::into_raw(Box::new(CodeHolder::new()));
}

/// Destroys a CodeHolder instance
#[no_mangle]
pub unsafe extern "C" fn rvm_codeholder_destroy(ch: *mut CodeHolder) {
    if !ch.is_null() {
        let ch = Box::from_raw(ch);
        drop(ch);
    }
}

/// Reads a bytecode file from a file path and outputs a CodeHolder instance. If successful,
/// returns a pointer to a CodeHolder instance; If not, returns a null pointer.
#[no_mangle]
pub extern "C" fn rvm_read_bytecode_file(path_char: *const c_char) -> *mut CodeHolder {
    if path_char.is_null() {
        return std::ptr::null_mut();
    }

    let path_str: &CStr = unsafe { CStr::from_ptr(path_char) };
    let path_slice: &str = match path_str.to_str() {
        Ok(v) => v,
        Err(_) => {
            return std::ptr::null_mut();
        }
    };

    let ch = match codereader::read_bytecode_file(path_slice) {
        Ok(v) => v,
        Err(_) => {
            return std::ptr::null_mut();
        }
    };

    return Box::into_raw(Box::new(ch));
}
