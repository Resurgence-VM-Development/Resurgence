/// Reference to the Cardinal system from Sword Art Online, Cardinal is the part of the runtime
/// that maintains a secure runtime.
///
/// Technically since it just hooks into the runtime and the runtime decides to use it, it can be
/// easily removed, if the programmer so wishes.
///
/// In the future when JIT compiling is added, Cardinal will also monitor how JITed functions
/// "deoptimize" themselves when returning back to bytecode.
pub(crate) struct Cardinal {
    /// Status of the runtime
    runtime_status: Status,
}

impl Cardinal {
    /// Creates a new `Cardinal` object
    #[inline]
    pub fn new() -> Cardinal {
        // By default everything is read and write
        Cardinal { 
            runtime_status: Status::NOT_STARTED, 
        }
    }
    
    /// Returns the current runtime status
    #[inline]
    pub fn runtime_security_status(&self) -> Status {
       self.runtime_status 
    }

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
}

/// Used to represent the status of the runtime
#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone)]
pub enum Status {
    NOT_STARTED,
    UNTAMPERED,
    TAMPERED,
}

