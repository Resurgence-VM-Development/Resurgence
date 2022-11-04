use crate::ext_func::resurgence_state::ResurgenceState;
use crate::internal::execution_engine::ExecutionEngine;
use crate::internal::interpreter::Interpreter;
use crate::CodeHolder;
use std::boxed::Box;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

/// Creates an instance of an Interpreter. If successful, returns a pointer to an Interpreter
/// instance. If this fails, it returns a null pointer. Consumes a CodeHolder.
#[no_mangle]
pub extern "C" fn rvm_interpreter_new(ch: *mut CodeHolder) -> *mut Interpreter {
    // Return null mutable pointer if codeholder is a null pointer
    if ch.is_null() {
        return std::ptr::null_mut();
    }

    // Create an Interpreter instance from the CodeHolder
    unsafe {
        let codeholder = *(Box::from_raw(ch));
        return Box::into_raw(Box::new(Interpreter::from(codeholder)));
    }
}

#[no_mangle]
pub extern "C" fn rvm_interpreter_register_function(
    inter: *mut Interpreter,
    callback: Option<extern "C" fn(&mut ResurgenceState) -> u8>,
    name_char: *const c_char,
) -> u8 {
    // Make sure parameters are non-null
    if inter.is_null() || name_char.is_null() {
        return 1;
    }

    let interpreter = unsafe { &mut *inter };

    let cbfunc = match callback {
        Some(f) => f,
        None => return 1,
    };

    let name_str: &CStr = unsafe { CStr::from_ptr(name_char) };
    let name_slice: &str = match name_str.to_str() {
        Ok(v) => v,
        Err(_) => {
            return 1;
        }
    };

    interpreter.register_native_function(cbfunc, String::from(name_slice));

    return 0;
}

/// Attempts to resolve all imports requested by the CodeHolder. If this succeeds, returns 0; If
/// this fails, it returns 1.
#[no_mangle]
pub extern "C" fn rvm_interpreter_resolve_imports(inter: *mut Interpreter) -> u8 {
    if inter.is_null() {
        return 1;
    }
    let interpreter = unsafe { &mut *inter };

    match interpreter.resolve_imports() {
        Ok(_) => return 0,
        Err(_) => return 1,
    };
}

#[no_mangle]
pub extern "C" fn rvm_interpreter_execute_function(
    inter: *mut Interpreter,
    name_char: *const c_char,
) -> u8 {
    // Make sure parameters are non-null
    if inter.is_null() || name_char.is_null() {
        return 1;
    }

    let interpreter = unsafe { &mut *inter };

    let name_str: &CStr = unsafe { CStr::from_ptr(name_char) };
    let name_slice: &str = match name_str.to_str() {
        Ok(v) => v,
        Err(_) => {
            return 1;
        }
    };

    match interpreter.execute_function(name_slice) {
        Ok(_) => return 0,
        Err(_) => return 1,
    }
}

/// Destroys an Interpreter instance
#[no_mangle]
pub unsafe extern "C" fn rvm_interpreter_destroy(inter: *mut Interpreter) {
    if !inter.is_null() {
        let inter = Box::from_raw(inter);
        drop(inter);
    }
}
