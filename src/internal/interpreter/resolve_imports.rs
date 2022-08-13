use std::io::{Error, ErrorKind};
use super::Interpreter;

impl Interpreter {
    pub fn resolve_imports(&mut self) -> Result<(), Error> {
        let imports = &self.code_holder.imports;
        for (index, name) in imports.iter().enumerate() {
            if self.rust_functions[index].name == *name {
                self.byte_to_interal[index] = index as u64;
            }
            else {
                for (internal_index, internal_name) in self.rust_functions.iter().enumerate() {
                    if *internal_name.name == *name {
                        self.byte_to_interal[index] = internal_index as u64;
                        return Result::Ok(());
                    }
                }
                return Err(Error::new(ErrorKind::NotFound, format!("Can not find function '{}' internally!", *name)))
            }
        }
        Result::Ok(())
    }
}