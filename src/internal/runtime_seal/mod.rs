/// This handles runtime validation to make sure the programmer isn't doing anything stupid
pub(crate) struct RunTimeSeal {
    pub untampered_runtime: bool,
}

impl RunTimeSeal {
    /// Sets the untampered_runtime variable to be true
    #[inline]
    pub fn set_runtime(&mut self) {
        self.untampered_runtime = true;
    }

    /// sets the untampered_runtime variable to be false
    #[inline]
    pub fn runtime_tampered(&mut self) {
        self.untampered_runtime = false;
    }

    pub fn new() -> RunTimeSeal {
        RunTimeSeal { untampered_runtime: false }
    }
}
