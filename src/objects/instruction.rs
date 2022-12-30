use super::register::{Register, RegisterReference, RegisterLocation};

/// `Instruction`: Represents instructions the built in Resurgence VM can use (this can be reused for any VM)
pub enum Instruction {
    /// Creates a `StackFrame` object and allocates n amount of registers
    /// 
    /// ```no_run
    /// 0 Alloc 5 // Stack frame containing 5 registers
    /// ```
    Alloc(u32),

    /// Allocates more registers in the current `StackFrame` object
    /// 
    /// ```no_run
    /// 0 Alloc 5 // Stack frame containing 5 registers
    /// 1 FrameAlloc 5 // Stack frame now contains 10 registers
    /// ```
    FrameAlloc(u32, RegisterLocation),

    /// Pops n amount of `StackFrame` objects
    /// 
    /// ```no_run
    /// 0 Alloc 5 // Creates a stack frame
    /// 1 Free 1 // Removes the top stack frame
    /// ```
    Free(u32),

    /// Frees n amount of registers in the current `StackFrame` object
    /// 
    /// ```no_run
    /// 0 Alloc 5 // Stack frame containing 5 registers
    /// 1 FrameFree 2 // Stack frame now contains 3 registers
    /// ```
    FrameFree(u32, RegisterLocation),

    /// Jumps n amount of operations
    /// 
    /// ```no_run
    /// 0 Alloc 5 // Stack frame containing 5 registers
    /// 1 Jump -1 // Jumps back to Alloc 
    /// ```
    Jump(i64),

    /// Jumps to index n by doing a recursive call of the execute_bytecode function
    /// 
    /// ```no_run
    /// 0 Call 1 // Calls the execute_bytecode function starting at index 1
    /// 1 Alloc 5 // Stack frame containing 5 registers
    /// 2 Free 1 // Frees the top stack frame
    /// ```
    Call(u64),

    /// Calls a Rust API function
    /// 
    /// ```no_run
    /// 0 ExtCall 0 // Calls a Rust API function assigned the ID of 0
    /// ```
    ExtCall(u64),

    /// Ends execution of execution_engine
    /// 
    /// ```no_run
    /// 0 FREE 1
    /// 1 RET // execution ends here
    /// ```
    Ret,

    /// Moves from one register to another
    /// 
    /// ```no_run
    /// 0 Mov 1, as_is, 0, as_is // Move the value stored in register 0 to register 1 
    /// ```
    Mov(Register, RegisterReference, Register, RegisterReference),

    /// Copies a value from one register to another
    /// 
    /// ```no_run
    /// 0 Cpy 1, as_is, 0, as_is // Copy the value stored in register 0 to register 1 
    /// ```
    Cpy(Register, RegisterReference, Register, RegisterReference),

    /// Stores the address of a register to another
    /// 
    /// ```no_run
    /// 0 Ref 1, as_is, 0, as_is // Store the address of register 0 in register 1
    /// ```
    Ref(Register, RegisterReference, Register, RegisterReference),

    /// Moves a value from a register to the stack
    /// 
    /// ```no_run
    /// 0 StackPush 0, as_is // Move the value stored in register 0 to the top of the stack
    /// ```
    StackPush(Register, RegisterReference),

    /// Pops the top of the stack
    /// 
    /// ```no_run
    /// 0 StackPop // Pop the top of the stack
    /// ```
    StackPop,

    /// Moves a value from the top of the stack to a register
    /// 
    /// ```no_run
    /// 0 StackMov 0, as_is // Move the top value of the stack to register 0
    /// ```
    StackMov(Register, RegisterReference),

    /// Adds 2 registers and stores it in the output
    /// 
    /// ```no_run
    /// 0 Add 0, 1, 2 // Add the values from registers 1 and 2 and store the output in register 0
    /// ```
    Add(Register, Register, Register),

    /// Subtract 2 registers and stores it in the output
    /// 
    /// ```no_run
    /// 0 Sub 0, 1, 2 // Subtract the values from registers 1 and 2 and store the output in register 0
    /// ```
    Sub(Register, Register, Register),

    /// Multiply 2 registers and stores it in the output
    /// 
    /// ```no_run
    /// 0 Mul 0, 1, 2 // Multiply the values from registers 1 and 2 and store the output in register 0
    /// ```
    Mul(Register, Register, Register),

    /// Divide 2 registers and stores it in the output
    /// 
    /// ```no_run
    /// 0 Div 0, 1, 2 // Divide the values from registers 1 and 2 and store the output in register 0
    /// ```
    Div(Register, Register, Register),

    /// Divides 2 registers and stores the remainder in the output
    /// 
    /// ```no_run
    /// 0 Div 0, 1, 2 // Divide the values from registers 1 and 2 and store the remainder in register 0
    /// ```
    Mod(Register, Register, Register),

    /// Checks if 2 registers are equal and jumps one operation if the condition is `true`
    /// 
    /// ```no_run
    /// Equal 0, 1 // Check if 0 and 1 are equal and jump if they are
    /// ```
    Equal(Register, Register),

    /// Checks if 2 registers are not equal and jumps one operation if the condition is `true`
    /// 
    /// ```no_run
    /// NotEqual 0, 1 // Check if 0 and 1 are not equal and jump if they are
    /// ```
    NotEqual(Register, Register),

    /// Checks if one register is greater then another and jumps one operation if the condition is `true`
    /// 
    /// ```no_run
    /// Greater 0, 1 // Check if 0 is greater than 1 and jumps one operation if it is
    /// ```
    Greater(Register, Register),

    ///  Checks if one register is less then another and jumps one operation if the condition is `true`
    /// 
    /// ```no_run
    /// Less 0, 1 // Check if 0 is less than 1 and jumps one operation if it is
    /// ```
    Less(Register, Register),

    /// Checks if one register is greater than or equal to another and jumps one operation if the condition is `true`
    /// 
    /// ```no_run
    /// GreaterEqual 0, 1 // Check if 0 is greater than or equal to 1 and jumps one operation if it is
    /// ```
    GreaterEqual(Register, Register),

    /// Checks if one register is less than or equal to another and jumps one operation if the condition is `true`
    /// 
    /// ```no_run
    /// LessEqual 0, 1 // Check if 0 is less than or equal to 1 and jumps one operation if it is
    /// ```
    LessEqual(Register, Register),
    
    /*
        Note that the following instructions under this line are just place holders and have not been implemented. I have
        yet to think of proper instructions for whatever these are meant to do, so please ignore them. I know these are
        interesting, but they're just placeholders.
            - StandingPad implementing a Vector type for Resurgence just before the first alpha
    */
    /// Unpacks a Vec to a range of registers to the stack
    ///
    /// ```no_run
    /// Unpack local(0) // Unpack a vector on the stack
    /// ```
    Unpack(Register),

    /// Packs a range of registers into a vector object
    ///
    /// Note: this is treated as LIFO. if the stack is (from top to bottom) [3, 2, 1], the vector
    /// will be [1, 2, 3]
    /// ```no_run
    /// Pack local(0) // Take the objects on the stack and pack them into a vector
    Pack(Register),
}
