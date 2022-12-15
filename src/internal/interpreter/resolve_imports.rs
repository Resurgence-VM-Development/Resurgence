use crate::{ResurgenceError, objects::resurgence_error::ResurgenceErrorKind};

use super::Interpreter;

impl Interpreter {
    /// Resolves any Rust functions used in the bytecode file by creating a "compatibility layer" based on indicies
    #[inline(never)]
    pub fn resolve_imports(&mut self) -> Result<(), ResurgenceError> {
        let imports = &self.code_holder.imports;
        self.code_holder.byte_to_interal.reserve(imports.len()); // We know the amount of functions being used, so let's take advantage of that
        for (_, name) in imports.iter().enumerate() {
            let mut success = false;
            for (internal_index, internal_name) in self.rust_functions.iter().enumerate() {
                if *internal_name.name == *name {
                    self.code_holder.byte_to_interal.push(internal_index as u64);
                    success = true;
                }
            }
            if !success {
                // Failed to find a matching import
                let err = ResurgenceError::from(ResurgenceErrorKind::MISSING_IMPORTS, &format!("Could not find function {} for it has not been registered", *name));
                err.add_trace(&format!("resolve_imports: line {}", line!()));
                return Err(err);
            }
        } 
        self.code_holder.resolved_imports = true;
        Result::Ok(())
    }
}
