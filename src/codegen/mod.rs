use crate::{CodeHolder, objects::{instruction::Instruction, register::{RegisterLocation, RegisterReference, Register}, constant::Constant}};

/// Represents a register location in RVM
#[allow(non_camel_case_types)]
pub enum RVMLocation {
    CONSTANT_POOL,
    ACCUMULATOR,
    GLOBAL,
    LOCAL
}

/// Represents a reference symbol in RVM
#[allow(non_camel_case_types)]
pub enum RVMReference {
    AS_IS,
    DEREFERENCE
}

/// Represents a register in RVM
pub struct RVMRegister(u32, RVMLocation);

/// Converts an abstracted location to an actual RVM register location
fn real_loc(moc_loc: RVMLocation) -> RegisterLocation {
    match moc_loc {
        RVMLocation::CONSTANT_POOL => RegisterLocation::ConstantPool,
        RVMLocation::ACCUMULATOR => RegisterLocation::Accumulator,
        RVMLocation::GLOBAL => RegisterLocation::Global,
        RVMLocation::LOCAL => RegisterLocation::Local,
    }
}

/// Converts an abstracted reference to an actual RVM reference
fn real_reference(moc_ref: RVMReference) -> RegisterReference {
    match moc_ref {
        RVMReference::AS_IS => RegisterReference::AsIs,
        RVMReference::DEREFERENCE => RegisterReference::Dereference
    }
}

/// Converts an abstracted register to an actual RVM register
fn real_register(moc_reg: RVMRegister) -> Register {
    Register(moc_reg.0, real_loc(moc_reg.1))
}

/// Generates an Alloc instruction 
///
/// amount (`u32`): The amount of registers to allocate
pub fn generate_alloc(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Alloc(amount)));
}

/// Generates a Free instruction 
///
/// amount (`u32`): The amount of blocks to free
pub fn generate_free(holder: &mut CodeHolder, amount: u32) {
    holder.instructions.push(Some(Instruction::Free(amount)));
}

/// Generates a FrameAlloc instruction
///
/// amount (`u32`): The amount of extra registers to allocate
pub fn generate_frame_alloc(holder: &mut CodeHolder, amount: u32, location: RVMLocation) {
    holder.instructions.push(Some(Instruction::FrameAlloc(amount, real_loc(location))));
}

/// Generates a FrameFree instruction
///
/// amount (`u32`): The amount of registers to free in the current StackFrame
pub fn generate_frame_free(holder: &mut CodeHolder, amount: u32, location: RVMLocation) {
   holder.instructions.push(Some(Instruction::FrameFree(amount, real_loc(location)))); 
}

/// Generates a Jump instruction
///
/// instructions (`i64`): The amount of instructions to jump (Negative numbers make the jump go
/// backwards)
pub fn generate_jump(holder: &mut CodeHolder, instructions: i64) {
    holder.instructions.push(Some(Instruction::Jump(instructions)));
}

/// Generates a call instruction
///
/// loc (`u64`): The instruction to jump to (based on by index)
pub fn generate_call(holder: &mut CodeHolder, loc: u64) {
    holder.instructions.push(Some(Instruction::Call(loc)));
}

/// Generates an ExtCall instruction
///
/// instruction_id (`u64`): The Rust/C function to call
pub fn generate_ext_call(holder: &mut CodeHolder, instruction_id: u64) {
    holder.instructions.push(Some(Instruction::ExtCall(instruction_id)));
}

/// Generates a Ret instruction
pub fn generate_return(holder: &mut CodeHolder) {
    holder.instructions.push(Some(Instruction::Ret));
}

/// Generates a Mov instruction
///
/// register_1 (`(RVMRegister, RVMReference)`): The destination register
/// register_2 (`(RVMRegister, RVMReference)`): The source register
pub fn generate_mov(holder: &mut CodeHolder, register_1: (RVMRegister, RVMReference), register_2: (RVMRegister, RVMReference)) {
    let reference_1 = real_reference(register_1.1);
    let reference_2 = real_reference(register_2.1);
    let real_reg_1 = real_register(register_1.0);
    let real_reg_2 = real_register(register_2.0);
    holder.instructions.push(Some(Instruction::Mov(real_reg_1, reference_1, real_reg_2, reference_2)));
}

/// Generates a Cpy instruction
///
/// register_1 (`(RVMRegister, RVMReference)`): The destination register
/// register_2 (`(RVMRegister, RVMReference)`): The source register
pub fn generate_cpy(holder: &mut CodeHolder, register_1: (RVMRegister, RVMReference), register_2: (RVMRegister, RVMReference)) {
    let reference_1 = real_reference(register_1.1);
    let reference_2 = real_reference(register_2.1);
    let real_reg_1 = real_register(register_1.0);
    let real_reg_2 = real_register(register_2.0);
    holder.instructions.push(Some(Instruction::Cpy(real_reg_1, reference_1, real_reg_2, reference_2)));
}

/// Generates a Ref instruction
///
/// register_1 (`(RVMRegister, RVMReference)`): The register that holds the reference
/// register_2 (`(RVMRegister, RVMReference)`): The register to create a reference to
pub fn generate_ref(holder: &mut CodeHolder, register_1: (RVMRegister, RVMReference), register_2: (RVMRegister, RVMReference)) {
    let reference_1 = real_reference(register_1.1);
    let reference_2 = real_reference(register_2.1);
    let real_reg_1 = real_register(register_1.0);
    let real_reg_2 = real_register(register_2.0);
    holder.instructions.push(Some(Instruction::Ref(real_reg_1, reference_1, real_reg_2, reference_2)));
}

/// Generates a StackPush instruction
///
/// register (`(RVMRegister, RVMReference)`): The register to push on the stack (copies the value so the register is safe to use afterwards)
pub fn generate_stack_push(holder: &mut CodeHolder, register: (RVMRegister, RVMReference)) {
    let reference = real_reference(register.1);
    let real_reg = real_register(register.0);
    holder.instructions.push(Some(Instruction::StackPush(real_reg, reference)));
}

/// Generates a StackPop instruction
pub fn generate_stack_pop(holder: &mut CodeHolder) {
    holder.instructions.push(Some(Instruction::StackPop));
} 

/// Generates a StackMov instruction
///
/// register (`(RVMRegister, RVMReference)`): The register to move the top of the stack to
pub fn generate_stack_mov(holder: &mut CodeHolder, register: (RVMRegister, RVMReference)) {
    let reference = real_reference(register.1);
    let real_reg = real_register(register.0);
    holder.instructions.push(Some(Instruction::StackMov(real_reg, reference)));
}

/// Generates an Add instruction
///
/// register_1 (`RVMRegister`): The destination register
/// register_2 (`RVMRegister`): The first source register
/// register_3 (`RVMRegister`): The second source register
pub fn generate_add(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Add(real_register(register_1), real_register(register_2), real_register(register_3))));
}


/// Generates an Sub instruction
///
/// register_1 (`RVMRegister`): The destination register
/// register_2 (`RVMRegister`): The first source register
/// register_3 (`RVMRegister`): The second source register
pub fn generate_sub(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Sub(real_register(register_1), real_register(register_2), real_register(register_3))));
}

/// generates an Mul instruction
///
/// register_1 (`rvmregister`): the destination register
/// register_2 (`rvmregister`): the first source register
/// register_3 (`rvmregister`): the second source register
pub fn generate_mul(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Mul(real_register(register_1), real_register(register_2), real_register(register_3))));
}

/// generates an Div instruction
///
/// register_1 (`rvmregister`): the destination register
/// register_2 (`rvmregister`): the first source register
/// register_3 (`rvmregister`): the second source register
pub fn generate_div(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Div(real_register(register_1), real_register(register_2), real_register(register_3))));
}

/// generates an Div instruction
///
/// register_1 (`rvmregister`): the destination register
/// register_2 (`rvmregister`): the first source register
/// register_3 (`rvmregister`): the second source register
pub fn generate_mod(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister, register_3: RVMRegister) {
    holder.instructions.push(Some(Instruction::Mod(real_register(register_1), real_register(register_2), real_register(register_3))));
}

/// Generates an Equal instruction
///
/// register_1 and register_2 (`RVMRegister`): The registers to compare for equality
pub fn generate_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::Equal(real_register(register_1), real_register(register_2))));
}

/// Generates an NotEqual instruction
///
/// register_1 and register_2 (`RVMRegister`): The registers to compare against for inequality
pub fn generate_not_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::NotEqual(real_register(register_1), real_register(register_2))));
}

/// Generates a Greater instruction
///
/// register_1 (`rvmregister`): the register to check
/// register_2 (`rvmregister`): the register to check against
pub fn generate_greater(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::Greater(real_register(register_1), real_register(register_2))));
}

/// Generates a Less instruction
///
/// register_1 (`rvmregister`): the register to check
/// register_2 (`rvmregister`): the register to check against
pub fn generate_less(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::Less(real_register(register_1), real_register(register_2))));
}

/// Generates a GreaterEqual instruction
///
/// register_1 (`rvmregister`): the register to check
/// register_2 (`rvmregister`): the register to check against
pub fn generate_greater_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::GreaterEqual(real_register(register_1), real_register(register_2))));
}

/// Generates a LessEqual instruction
///
/// register_1 (`rvmregister`): the register to check
/// register_2 (`rvmregister`): the register to check against
pub fn generate_less_equal(holder: &mut CodeHolder, register_1: RVMRegister, register_2: RVMRegister) {
    holder.instructions.push(Some(Instruction::LessEqual(real_register(register_1), real_register(register_2))));
}

/// Returns the index of the last object in a vector as a u32
///
/// $vec: the vector in question
macro_rules! get_index {
    ($vec:expr) => {
        ($vec.len() - 1) as u32
    };
}

/// Generates a Integer constant, returns an `RVMRegister` object
///
/// value (`i64`): the integer to use in the constant pool
pub fn generate_int_constant(holder: &mut CodeHolder, value: i64) -> RVMRegister {
    holder.constant_pool.push(Constant::Int(value));
    RVMRegister(get_index!(holder.constant_pool), RVMLocation::CONSTANT_POOL)
}

/// Generates a Double constant, returns an `RVMRegister` object
///
/// value (`f64`): the integer to use in the constant pool
pub fn generate_double_constant(holder: &mut CodeHolder, value: f64) -> RVMRegister {
    holder.constant_pool.push(Constant::Double(value));
    RVMRegister(get_index!(holder.constant_pool), RVMLocation::CONSTANT_POOL)
}

/// Generates a String constant, returns an `RVMRegister` object
///
/// value (`String`): the integer to use in the constant pool
pub fn generate_string_constant(holder: &mut CodeHolder, value: String) -> RVMRegister {
    holder.constant_pool.push(Constant::String(value));
    RVMRegister(get_index!(holder.constant_pool), RVMLocation::CONSTANT_POOL)
}

/// Generates a Boolean constant, returns an `RVMRegister` object
///
/// value (`bool`): the integer to use in the constant pool
pub fn generate_bool_constant(holder: &mut CodeHolder, value: bool) -> RVMRegister {
    holder.constant_pool.push(Constant::Boolean(value));
    RVMRegister(get_index!(holder.constant_pool), RVMLocation::CONSTANT_POOL)
}

