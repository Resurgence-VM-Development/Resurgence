/// This handles runtime validation to make sure the programmer isn't doing anything stupid
pub(crate) struct RunTimeSeal {
    /// Status of the runtime
    runtime_status: Status,
    /// Permission level for Rust and C functions (ie. can we add new functions at runtime?)
    pub(crate) rust_c_permissions: Permissions,
    /// Permission level for global registers
    pub(crate) global_permissions: Permissions,
    /// Permission level for the call stack 
    pub(crate) call_stack_permissions: Permissions,
    /// Permission level for the constant stack
    pub(crate) constant_stack_permissions: Permissions,
}

impl RunTimeSeal {
    /// Creates a new `RunTimeSeal` object
    #[inline]
    pub fn new() -> RunTimeSeal {
        // By default everything is read and write
        RunTimeSeal { 
            runtime_status: Status::NOT_STARTED, 
            rust_c_permissions: Permissions::WRITE,
            global_permissions: Permissions::WRITE,
            call_stack_permissions: Permissions::WRITE,
            constant_stack_permissions: Permissions::WRITE,
        }
    }
    
    /* 
    RUNTIME STATUS FUNCTIONS
    */
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
    
   
    /* 
    ALL PERMISSION RELATED FUNCTIONS HERE
    */
    /// Sets the permission level for Rust and C functions 
    /// level (`Permissions`): The desired permission level
    #[inline]
    pub fn set_rust_c_perms(&mut self, level: Permissions) {
        if self.runtime_status != Status::NOT_STARTED {
            self.runtime_tampered();
            return;
        }
        self.rust_c_permissions = level;
    }

    #[inline]
    pub fn set_global_perms(&mut self, level: Permissions) {

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

/// All permissions possible
#[allow(non_camel_case_types)]
pub(crate) enum Permissions {
    READ,
    WRITE,
}


