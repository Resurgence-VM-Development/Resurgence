use crate::api::ext_func::resurgence_state::ResurgenceState;
use std::io::Error;

pub struct RustFunc {
    pub name: String,
    pub func: fn(&mut ResurgenceState) -> Result<(), Error>,
}
