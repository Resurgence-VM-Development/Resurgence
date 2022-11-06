use crate::{CodeHolder, objects::{instruction::Instruction, register::{RegisterLocation, RegisterReference, Register}}};

pub enum RVMLocation {
    GLOBAL,
    LOCAL
}

pub enum RVMReference {
    AsIs,
    DEREFERENCE
}

pub struct RVMRegister(u32, RVMLocation);

fn real_loc(moc_loc: RVMLocation) -> RegisterLocation {
    match moc_loc {
        RVMLocation::GLOBAL => RegisterLocation::Global,
        RVMLocation::LOCAL => RegisterLocation::Local
    }
}

fn real_reference(moc_ref: RVMReference) -> RegisterReference {
    match moc_ref {
        RVMReference::AsIs => RegisterReference::AsIs,
        RVMReference::DEREFERENCE => RegisterReference::Dereference
    }
}

fn real_register(moc_reg: RVMRegister) -> Register {
    Register(moc_reg.0, real_loc(moc_reg.1))
}

pub fn generate_alloc(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Alloc(amount)));
}

pub fn generate_free(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Free(amount)));
}

pub fn generate_frame_alloc(holder: &mut CodeHolder, amount: u32, location: RVMLocation) {
    holder.instructions.push(Some(Instruction::FrameAlloc(amount, real_loc(location))));
}

pub fn generate_frame_free(holder: &mut CodeHolder, amount: u32, location: RVMLocation) {
   holder.instructions.push(Some(Instruction::FrameFree(amount, real_loc(location)))); 
}

pub fn generate_jump(holder: &mut CodeHolder, instructions: i64) {
    holder.instructions.push(Some(Instruction::Jump(instructions)));
}

pub fn generate_call(holder: &mut CodeHolder, loc: u64) {
    holder.instructions.push(Some(Instruction::Call(loc)));
}

pub fn generate_ext_call(holder: &mut CodeHolder, instruction_id: u64) {
    holder.instructions.push(Some(Instruction::ExtCall(instruction_id)));
}

pub fn generate_return(holder: &mut CodeHolder) {
    holder.instructions.push(Some(Instruction::Ret));
}

pub fn generate_mov(holder: &mut CodeHolder, register_1: (RVMRegister, RVMReference), register_2: (RVMRegister, RVMReference)) {
    let reference_1 = real_reference(register_1.1);
    let reference_2 = real_reference(register_2.1);
    let real_reg_1 = real_register(register_1.0);
    let real_reg_2 = real_register(register_2.0);
    holder.instructions.push(Some(Instruction::Mov(real_reg_1, reference_1, real_reg_2, reference_2)));
}

pub fn generate_cpy(holder: &mut CodeHolder, register_1: (RVMRegister, RVMReference), register_2: (RVMRegister, RVMReference)) {
    let reference_1 = real_reference(register_1.1);
    let reference_2 = real_reference(register_2.1);
    let real_reg_1 = real_register(register_1.0);
    let real_reg_2 = real_register(register_2.0);
    holder.instructions.push(Some(Instruction::Cpy(real_reg_1, reference_1, real_reg_2, reference_2)));
}

pub fn generate_ref(holder: &mut CodeHolder, register_1: (RVMRegister, RVMReference), register_2: (RVMRegister, RVMReference)) {
    let reference_1 = real_reference(register_1.1);
    let reference_2 = real_reference(register_2.1);
    let real_reg_1 = real_register(register_1.0);
    let real_reg_2 = real_register(register_2.0);
    holder.instructions.push(Some(Instruction::Ref(real_reg_1, reference_1, real_reg_2, reference_2)));
}

pub fn generate_stack_push(holder: &mut CodeHolder, register: (RVMRegister, RVMReference)) {
    let reference = real_reference(register.1);
    let real_reg = real_register(register.0);
    holder.instructions.push(Some(Instruction::StackPush(real_reg, reference)));
}

pub fn generate_stack_pop(holder: &mut CodeHolder) {
    holder.instructions.push(Some(Instruction::StackPop));
} 

pub fn generate_stack_mov(holder: &mut CodeHolder, register: (RVMRegister, RVMReference)) {
    let reference = real_reference(register.1);
    let real_reg = real_register(register.0);
    holder.instructions.push(Some(Instruction::StackMov(real_reg, reference)));
}

pub fn generate_add(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Add(real_register(register_1), real_register(register_2), real_register(register_3))));
}

pub fn generate_sub(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Sub(real_register(register_1), real_register(register_2), real_register(register_3))));
}

pub fn generate_mul(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Mul(real_register(register_1), real_register(register_2), real_register(register_3))));
}

pub fn generate_div(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Div(real_register(register_1), real_register(register_2), real_register(register_3))));
}

pub fn generate_mod(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Mod(real_register(register_1), real_register(register_2), real_register(register_3))));
}

pub fn generate_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::Equal(real_register(register_1), real_register(register_2))));
}

pub fn generate_not_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::NotEqual(real_register(register_1), real_register(register_2))));
}

pub fn generate_greater(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::Greater(real_register(register_1), real_register(register_2))));
}

pub fn generate_less(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::Less(real_register(register_1), real_register(register_2))));
}

pub fn generate_greater_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::GreaterEqual(real_register(register_1), real_register(register_2))));
}

pub fn generate_less_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::LessEqual(real_register(register_1), real_register(register_2))));
}
