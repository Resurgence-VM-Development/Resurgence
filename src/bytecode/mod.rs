/*!
# Bytecode API
This module provides functions for converting to/from raw bytecode data and a [`crate::CodeHolder`]
instance.

# Examples
Read a bytecode file and convert it into a [`crate::CodeHolder`] instance:
```no_run
use resurgence::bytecode;

let holder = bytecode::read_bytecode_file("path/to/bytecode.rvm").unwrap();
```

Convert a [`crate::CodeHolder`] to bytecode and write it to a file:
```no_run
use resurgence::{bytecode, CodeHolder};

let holder = CodeHolder::new();
bytecode::write_bytecode_file(&holder, "path/to/destination.rvm").unwrap();
```
*/

pub(crate) mod codereader;
pub(crate) mod codewriter;
mod parser_constants;

pub use codereader::{read_bytecode, read_bytecode_file};
pub use codewriter::{write_bytecode, write_bytecode_file};
