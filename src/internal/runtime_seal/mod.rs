/// This handles runtime validation to make sure the programmer isn't doing anything stupid
pub(crate) struct RunTimeSeal {
    untampered_runtime: bool,
}

impl RunTimeSeal {
    #[inline]
    pub fn set_runtime(&mut self) {
        self.untampered_runtime = true;
    }
    #[inline]
    pub fn runtime_tampered(&mut self) {
        self.untampered_runtime = false;
    }
}
