use crate::api::ext_func::resurgence_state::ResurgenceState;
use std::ffi::CString;
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
