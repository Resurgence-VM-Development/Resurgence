use std::{collections::HashMap};
use crate::objects::codeholder::CodeHolder;
mod generate_instruction;

/// `CodeBuilder<'a>`: Helps contruct bytecode to be used by the VM
struct CodeBuilder<'a> {
    /// Reference to a `CodeHolder` 
    code_holder: &'a mut CodeHolder,

    /// Holds function symbols 
    func_symbols: HashMap<String, usize>,
    
    /// Holds variable symbols
    var_symbols: Vec<HashMap<String, u32>>
}
