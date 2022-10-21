use super::constant::Constant;
use super::instruction::Instruction;
use std::collections::HashMap;

/// A CodeHolder represents a set of executable instructions and a pool of immutable data for an
/// [`crate::Interpreter`] to use at runtime.
pub struct CodeHolder {
    /// A [`Vec`] of executable instructions
    pub instructions: Vec<Option<Instruction>>,

    /// A pool of immutable data that is available to the VM at runtime.
    pub constant_pool: Vec<Constant>,

    /// A list of imports that are required to properly link with the application
    pub(crate) imports: Vec<String>,

    /// A list of calls that the code exports and makes available to the application at runtime.
    pub(crate) exports: HashMap<String, u64>,
}

impl CodeHolder {
    /// Creates a new CodeHolder instance
    pub fn new() -> CodeHolder {
        CodeHolder {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
            imports: Vec::new(),
            exports: HashMap::new(),
        }
    }

    pub fn has_export(&self, func_name: &String) -> bool {
        self.exports.contains_key(func_name)
    }
}

impl Default for CodeHolder {
    fn default() -> Self {
        Self::new()
    }
}
