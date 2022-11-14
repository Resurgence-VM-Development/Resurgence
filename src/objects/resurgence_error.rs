use std::io::ErrorKind;

use crate::objects::register::RegisterLocation;

use super::{stackframe::StackFrame, instruction::Instruction, constant::Constant};

struct ResurgenceError {
    error_type: ErrorKind,
    error_message: String,
    call_stack: Vec<StackFrame>,
    instruction: Instruction,
    instruction_pointer: usize
}

impl ResurgenceError {
    fn print_error(&self) {
        println!("{}", self.error_message);
        
        println!("Unwinding callstack with {} frames", self.call_stack.len());
        for frame in &self.call_stack {
            println!("------------------------------------------------------");
            let mut reg_index: u32 = 0;
            for register in &frame.registers {
                match register {
                    Some(value) => {
                        match value {
                            Constant::Int(int_val) => println!("Register {}, i64 Constant: {}", reg_index, int_val),
                            Constant::Double(double_val) => println!("Register {}, f64 Constant: {}", reg_index, double_val),
                            Constant::String(string_val) => println!("Register {}, String Constant: {}", reg_index, string_val),
                            Constant::Boolean(bool_val) => println!("Register {}, bool Constant: {}", reg_index, if *bool_val {"true"} else {"false"}),
                            Constant::Address(address_value) => println!("Register {}, Register constant: index({}) location({})", reg_index, address_value.0, match address_value.1 {
                                RegisterLocation::ConstantPool => "Constant Pool",
                                RegisterLocation::Accumulator => "Accumulator",
                                RegisterLocation::Global => "Global",
                                RegisterLocation::Local => "Local"
                            }),
                        }
                    },
                    None => println!("Register {}, None", reg_index),
                }  
                reg_index += 1;
            }
        println!("------------------------------------------------------");
        }
    }
}
