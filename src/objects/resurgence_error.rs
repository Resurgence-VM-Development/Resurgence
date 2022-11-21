use crate::internal::interpreter::imports::RustFunc;

use super::{stackframe::StackFrame, instruction::Instruction, constant::Constant};

/// All types of errors in Resurgence
#[allow(non_camel_case_types)]
pub(crate) enum ResurgenceErrorKind {
    /// Operation is invalid (ex. using a ConsantPool value in a `MOV` or `REF` instruction)
    INVALID_OPERATION,
    /// Register set to none when trying to access it (ex. `ADD`)
    MEMORY_ADDRESS_NONE,
    /// Integer overflow
    OVERFLOW,
    /// Runtime safety violated (ex. trying to register a function in a Rust API function)
    RUNTIME_SEAL_TAMPERED,
    /// Out of bounds when accessing a register
    REGISTER_OUT_OF_BOUNDS,
    /// When imports are not resolved
    MISSING_IMPORTS,
}

/// Represents a exception and stores the current state of the runtime, including callstack,
/// current instruction, and the parameters of the instruction in question
pub struct ResurgenceError {
    /// The type of error
    error_type: ResurgenceErrorKind,
    /// Error message 
    error_message: String,
    /// Call stack at the time of exception
    call_stack: Vec<StackFrame>,
    /// Constant stack at the time of exception
    constant_stack: Vec<Constant>,
    /// All registered functions and their indexes
    rust_and_native_fns: Vec<RustFunc>,
    /// Instruction at the time of exception
    instruction: Instruction,
    /// Instruction index at time of exception
    instruction_pointer: usize,
    /// Recursion depth 
    recursion_depth: usize,

}

impl ResurgenceError {
    pub(crate) fn from(error_type: ResurgenceErrorKind, error_message: &str) -> ResurgenceError {
        ResurgenceError {
            error_type,
            error_message: error_message.to_string(),
            call_stack: Vec::new(),
            constant_stack: Vec::new(),
            rust_and_native_fns: Vec::new(),
            instruction: Instruction::Ret,
            instruction_pointer: 0,
            recursion_depth: 0
        }
    }

    /// Prints and core dumps the error for the programmer to debug with
    pub fn throw_error(&self) {
        println!("Exception thrown!: {}", self.error_message);
        
        // Unwind the callstack
        println!("Unwinding callstack with {} frames", self.call_stack.len());
        for frame in &self.call_stack {
            println!("------------------------------------------------------");
            let mut reg_index: u32 = 0;
            for register in &frame.registers {
                match register {
                    Some(value) => {
                        println!("Register {}: {}", reg_index, value.type_as_string());
                    },
                    None => println!("Register {}, None", reg_index),
                }  
                reg_index += 1;
            }
        println!("------------------------------------------------------");
        }
        
        // Print every function registered in the runtime
        let mut func_index = 0;
        for function in &self.rust_and_native_fns {
            if function.native {
                println!("C function {} at address {}", function.name, func_index)
            } else {
                println!("Rust function {} at address {}", function.name, func_index);
            }
            func_index += 1;
        }
    }
    
    /// Sets the instruction field to the instruction that the VM was on when the error occured
    ///
    /// instruction (`Instruction`): The instruction in question
    #[inline]
    pub(crate) fn set_instruction(&mut self, instruction: Instruction) {
        self.instruction = instruction;
    }

    /// Sets thw instruction pointer to the index value that the VM was on when the error occured
    ///
    /// ip (`usize`): The instruction pointer value
    #[inline]
    pub(crate) fn set_ip(&mut self, ip: usize) {
        self.instruction_pointer = ip;
    }

    /// Passes the current state of the callstack
    ///
    /// call_stack (`Vec<StackFrame`): The callstack
    #[inline]
    pub(crate) fn set_call_stack(&mut self, call_stack: Vec<StackFrame>) {
        self.call_stack = call_stack;
    }

    /// Passes the current state of the constant stack
    ///
    /// constant_stack (`Vec<Constant>`): The constant stack
    #[inline]
    pub(crate) fn set_constant_stack(&mut self, constant_stack: Vec<Constant>) {
        self.constant_stack = constant_stack;
    }


    /// Passes API functions
    ///
    /// functions (`Vec<RustFunc>`): API functions
    #[inline]
    pub(crate) fn set_functions(&mut self, functions: Vec<RustFunc>) {
        self.rust_and_native_fns = functions;
    }

    /// Passes the current recursion depth 
    ///
    /// rd (`usize`): Recursion depth
    #[inline]
    pub(crate) fn set_recursion_depth(&mut self, rd: usize) {
        self.recursion_depth = rd;
    }
}
