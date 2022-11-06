use crate::{CodeHolder, objects::{instruction::Instruction, register::RegisterLocation}};

pub enum RVMLocation {
    GLOBAL,
    LOCAL
}

pub fn generate_alloc(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Alloc(amount)));
}

pub fn generate_free(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Free(amount)));
}

pub fn generate_frame_alloc(holder: &mut CodeHolder, amount: u32, location: RVMLocation) {
    match location {
        RVMLocation::GLOBAL => holder.instructions.push(Some(Instruction::FrameAlloc(amount, RegisterLocation::Global))),
        RVMLocation::LOCAL => holder.instructions.push(Some(Instruction::FrameAlloc(amount, RegisterLocation::Local))),
    }
}

pub fn generate_frame_free(holder: &mut CodeHolder, amount: u32, location: RVMLocation) {
    match location {
        RVMLocation::GLOBAL => holder.instructions.push(Some(Instruction::FrameFree(amount, RegisterLocation::Global))),
        RVMLocation::LOCAL => holder.instructions.push(Some(Instruction::FrameFree(amount, RegisterLocation::Local))),
    }
}

pub fn generate_jump(holder: &mut CodeHolder, instructions: i64) {
    holder.instructions.push(Some(Instruction::Jump(instructions)));
}

pub fn generate_call(holder: &mut CodeHolder, loc: u32) {
    holder.instructions.push(Some(Instruction::Call(loc)));
}

pub fn generate_ext_call(holder: &mut CodeHolder, instruction_id: u32) {
    holder.instructions.push(Some(Instruction::ExtCall(instruction_id)));
}

pub fn generate_return(holder: &mut CodeHolder) {
    holder.instructions.push(Some(Instruction::Ret));
}
