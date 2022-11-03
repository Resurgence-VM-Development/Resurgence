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


