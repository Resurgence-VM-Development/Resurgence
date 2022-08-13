use std::io::Error;
use crate::api::ext_func::resurgence_state::ResurgenceState;

pub struct RustFunc{
    pub name: String, 
    pub func: fn(ResurgenceState) -> Result<(), Error>
}