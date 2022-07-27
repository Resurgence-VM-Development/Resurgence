use super::register::{Register, RegisterReference};

/// `Bytecode`: Represents instructions the built in Resurgence VM can use (this can be reused for any VM)
/// 
/// Possible Values: 
/// * `Alloc(u32)`: Preallocates memory in the vector stored in a `StackFrame` object 
/// * `Free(u32)`: Pops n amount of `StackFrame` objects
/// * `Jump(u64)`: Jumps n amount of operations
/// * `Mov(Register, RegisterReference, Register, RegisterReference)`: Moves from one register to another
/// * `Cpy(Register, RegisterReference, Register, RegisterReference)`: Copies a value from one register to another
/// * `Ref(Register, RegisterReference, Register, RegisterReference)`: Stores the address of a register to another
/// * `Add(Register, Register, RegisterReference, Register, RegisterReference)`: Adds 2 registers and stores it in the output
/// * `Sub(Register, Register, RegisterReference, Register, RegisterReference)`: Subtracts 2 registers and stores it in the output
/// * `Mul(Register, Register, RegisterReference, Register, RegisterReference)`: Multiples 2 registers and stores it in the output
/// * `Div(Register, Register, RegisterReference, Register, RegisterReference)`: Divides 2 registers and stores it in the output
/// * `Equal(Register, RegisterReference, Register, RegisterReference)`: Checks if 2 registers are equal and jumps one operation if the condition is `true`
/// * `NotEqual(Register, RegisterReference, Register, RegisterReference)`: Checks if 2 registers are not equal and jumps one operation if the condition is `true`
/// * `Greater(Register, RegisterReference, Register, RegisterReference)`: Checks if one register is greater then another and jumps one operation if the condition is `true`
/// * `Less(Register, RegisterReference, Register, RegisterReference)`: Checks if one register is less then another and jumps one operation if the condition is `true`
/// * `GreaterEqual(Register, RegisterReference, Register, RegisterReference)`: Checks if one register is greater than or equal to another and jumps one operation if the condition is `true`
/// * `LessEqual(Register, RegisterReference, Register, RegisterReference)`: Checks if one register is less than or equal to another and jumps one operation if the condition is `true`
pub enum ByteCode {
    Alloc(u32),
    Free(u32),
    Jump(u64),

    Mov(Register, RegisterReference, Register, RegisterReference),
    Cpy(Register, RegisterReference, Register, RegisterReference),
    Ref(Register, RegisterReference, Register, RegisterReference),

    Add(Register, Register, RegisterReference, Register, RegisterReference),
    Sub(Register, Register, RegisterReference, Register, RegisterReference),
    Mul(Register, Register, RegisterReference, Register, RegisterReference),
    Div(Register, Register, RegisterReference, Register, RegisterReference),

    Equal(Register, RegisterReference, Register, RegisterReference),
    NotEqual(Register, RegisterReference, Register, RegisterReference),
    Greater(Register, RegisterReference, Register, RegisterReference),
    Less(Register, RegisterReference, Register, RegisterReference),
    GreaterEqual(Register, RegisterReference, Register, RegisterReference),
    LessEqual(Register, RegisterReference, Register, RegisterReference),
}