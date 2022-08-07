use super::CodeBuilder;
use crate::objects::instruction::Instruction;

impl CodeBuilder<'_> {
    pub fn generate_alloc(&mut self, block_count: u32) {
        self.code_holder.0.push(Instruction::Alloc(block_count));
    }
    pub fn generate_free(&mut self, block_count: u32) {
        self.code_holder.0.push(Instruction::Free(block_count));
    }
}