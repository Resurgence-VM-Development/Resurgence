use super::Interpreter;
use std::io::{Error, ErrorKind};

impl Interpreter {
    /// Resolves any Rust functions used in the bytecode file by creating a "compatibility layer" based on indicies
    pub fn resolve_imports(&mut self) -> Result<(), Error> {
        let imports = &self.code_holder.imports;
        for (index, name) in imports.iter().enumerate() {
            let mut success = false;
            for (internal_index, internal_name) in self.rust_functions.iter().enumerate() {
                if *internal_name.name == *name {
                    self.byte_to_interal.insert(index, internal_index as u64);
                    success = true;
                }
            }
            if !success {
                // Failed to find a matching import
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Can not find function '{}' internally!", *name),
                ));
            }
        }
        Result::Ok(())
    }
}
