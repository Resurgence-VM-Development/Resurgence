/// This handles runtime validation to make sure the programmer isn't doing anything stupid
pub(crate) struct RunTimeSeal {
    runtime_status: Status,
}

impl RunTimeSeal {
    /// Sets the untampered_runtime variable to be true
    #[inline]
    pub fn set_runtime(&mut self) {
        self.runtime_status = Status::UNTAMPERED;
    }

    /// sets the untampered_runtime variable to be false
    #[inline]
    pub fn runtime_tampered(&mut self) {
        self.runtime_status = Status::TAMPERED;
    }
    
    #[inline]
    pub fn new() -> RunTimeSeal {
        RunTimeSeal { runtime_status: Status::UNTAMPERED }
    }

    #[inline]
    pub fn runtime_security_status(&self) -> Status {
       self.runtime_status 
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Status {
    UNTAMPERED,
    TAMPERED,
}
