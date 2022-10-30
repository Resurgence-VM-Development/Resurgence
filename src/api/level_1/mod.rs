use crate::{CodeHolder, objects::instruction::Instruction};

pub fn generate_alloc(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Alloc(amount)));
}
