use super::CodeBuilder;
use crate::objects::instruction::Instruction;

// TODO: Finish this
impl CodeBuilder<'_> {
    pub fn generate_alloc(&mut self, block_count: u32) {
        self.code_holder.instructions.push(Instruction::Alloc(block_count));
    }
    pub fn generate_free(&mut self, block_count: u32) {
        self.code_holder.instructions.push(Instruction::Free(block_count));
    }
    pub fn generate_jump(&mut self, operation_count: i64) {
        self.code_holder.instructions.push(Instruction::Jump(operation_count));
    }
    pub fn generate_call(&mut self, function_addr: &String) {
        let address = *self.func_symbols.get(function_addr).expect("Non-existant label") as u64;
        self.code_holder.instructions.push(Instruction::Call(address));
    }
}