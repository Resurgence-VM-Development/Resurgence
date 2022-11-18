use crate::ext_func::resurgence_state::ResurgenceState;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rvm_state_get_integer(state: *mut ResurgenceState, out_value: *mut i64) -> u8 {
    if state.is_null() || out_value.is_null() {
        return 1;
    }

    let state = unsafe { &mut *state };

    match state.get_i64() {
        Ok(v) => {
            unsafe { *out_value = v };
            return 0;
        }
        Err(_) => return 1,
    }
}

#[no_mangle]
pub extern "C" fn rvm_state_get_float(state: *mut ResurgenceState, out_value: *mut f64) -> u8 {
    if state.is_null() || out_value.is_null() {
        return 1;
    }

    let state = unsafe { &mut *state };

    match state.get_f64() {
        Ok(v) => {
            unsafe { *out_value = v };
            return 0;
        }
        Err(_) => return 1,
    }
}

#[no_mangle]
pub extern "C" fn rvm_state_get_string(
    state: *mut ResurgenceState,
    out_value: *mut *mut c_char,
) -> u8 {
    if state.is_null() || out_value.is_null() {
        return 1;
    }

    let state = unsafe { &mut *state };

    match state.get_string() {
        Ok(v) => {
            let c_string = match CString::new(v) {
                Ok(x) => x,
                Err(_) => return 1,
            };
            unsafe {
                *out_value = c_string.into_raw();
            }
            return 0;
        }
        Err(_) => return 1,
    }
}

#[no_mangle]
pub extern "C" fn rvm_state_get_bool(state: *mut ResurgenceState, out_value: *mut u8) -> u8 {
    if state.is_null() || out_value.is_null() {
        return 1;
    }

    let state = unsafe { &mut *state };

    match state.get_bool() {
        Ok(v) => {
            unsafe {
                *out_value = match v {
                    true => 1,
                    false => 0,
                }
            };
            return 0;
        }
        Err(_) => return 1,
    }
}

#[no_mangle]
pub extern "C" fn rvm_state_push_integer(state: *mut ResurgenceState, value: i64) -> u8 {
    if state.is_null() {
        return 1;
    }

    let state = unsafe { &mut *state };
    state.push_i64(value);
    return 0;
}

#[no_mangle]
pub extern "C" fn rvm_state_push_float(state: *mut ResurgenceState, value: f64) -> u8 {
    if state.is_null() {
        return 1;
    }

    let state = unsafe { &mut *state };
    state.push_f64(value);
    return 0;
}

#[no_mangle]
pub extern "C" fn rvm_state_push_string(state: *mut ResurgenceState, value: *const c_char) -> u8 {
    if state.is_null() || value.is_null() {
        return 1;
    }
    let state = unsafe { &mut *state };

    let v_str: &CStr = unsafe { CStr::from_ptr(value) };
    let v_slice: &str = match v_str.to_str() {
        Ok(v) => v,
        Err(_) => {
            return 1;
        }
    };

    state.push_string(v_slice.clone().to_owned());

    return 0;
}

#[no_mangle]
pub extern "C" fn rvm_state_push_bool(state: *mut ResurgenceState, value: u8) -> u8 {
    if state.is_null() {
        return 1;
    }
    let state = unsafe { &mut *state };
    match value {
        0 => {
            state.push_bool(false);
            return 0;
        }
        _ => {
            state.push_bool(true);
            return 0;
        }
    }
}
