use super::constant::Constant;
use super::instruction::Instruction;

/// A CodeHolder represents a set of executable instructions and a pool of immutable data for an
/// [`crate::Interpreter`] to use at runtime.
pub struct CodeHolder {
    /// A [`Vec`] of executable instructions
    pub instructions: Vec<Instruction>,
    /// A pool of immutable data that is available to the VM at runtime.
    pub constant_pool: Vec<Constant>,
}

impl CodeHolder {
    /// Creates a new CodeHolder instance
    pub fn new() -> CodeHolder {
        CodeHolder {
            instructions: Vec::new(),
            constant_pool: Vec::new(),
        }
    }
}

impl Default for CodeHolder {
    fn default() -> Self {
        Self::new()
    }
}
