use super::super::super::{interpreter::Interpreter};
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};


pub fn MovRegisters(interpreter: &mut Interpreter, dst_reg: &Register, dst_reg_ref: &RegisterReference, src_reg: &Register, src_reg_ref: &RegisterReference) {
    let Register(mut dst_index, mut dst_loc) = dst_reg; 
    let Register(mut src_index, mut src_loc) = src_reg; 
    let mut dst_index_usize = dst_index as usize;
    let mut src_index_usize = src_index as usize;

    // Dereferenceing values for the destination register
    if *dst_reg_ref == RegisterReference::Dereference {
        match dst_loc {
            RegisterLocation::Global => {
                // Take the register out of global memory and dereference
                let register = interpreter.global[dst_index_usize].as_ref().expect("Non-existant register!");
                if let Constant::Address(dref_reg) = register {
                    Register(dst_index, dst_loc) = *dref_reg;
                }
            },
            RegisterLocation::Local => {
                // Get the register, dereference it, and then store it in the src_index and src_loc values
                let stack_frame = interpreter.call_stack.last_mut().unwrap();

                let src_reg = stack_frame.registers[dst_index_usize].as_ref().expect("Non-existant register to dereference!");
                if let Constant::Address(dref_reg) = src_reg {
                    Register(dst_index, dst_loc) = *dref_reg;
                } else {
                    panic!("Must dereference a address!")
                }
            },
            _ => panic!("Invalid dereference operation!")
        }
        dst_index_usize = dst_index as usize;
    }
    
    // Dereferencing values for the source register
    if *src_reg_ref == RegisterReference::Dereference {
        match src_loc {
            RegisterLocation::Global => {
                // Take the register out of global memory and dereference
                let register = interpreter.global[src_index_usize].as_ref().expect("Non-existant register!");
                if let Constant::Address(dref_reg) = register {
                    Register(src_index, src_loc) = *dref_reg;
                } else {
                    panic!("Must dereference a address!")
                }
            },
            RegisterLocation::Local => {
                // Get the register, dereference it, and then store it in the src_index and src_loc values
                let stack_frame = interpreter.call_stack.last_mut().unwrap();

                let src_reg = stack_frame.registers[src_index_usize].as_ref().expect("Non-existant register to dereference!");
                if let Constant::Address(dref_reg) = src_reg {
                    Register(src_index, src_loc) = *dref_reg;
                } else {
                    panic!("Must dereference a address!")
                }
            },
            _ => panic!("Invalid dereference operation!")
        }
        src_index_usize = src_index as usize;
    }

    match (dst_loc, src_loc) {
        (RegisterLocation::Accumulator, RegisterLocation::Global) => {
            let src_register = interpreter.global[src_index_usize].take(); // take the value from global memory
            
            // Start doing the move if src_register is not None
            if let Some(src_val) = src_register {
                match src_val {
                    Constant::Int(src_int) => {
                        interpreter.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        interpreter.accumulator = src_double;
                    }
                    _ => panic!("Invalid move to accumulator register!"),
                }
            } else {
                panic!("Segmentation Fault!")
            }
        },
        (RegisterLocation::Accumulator, RegisterLocation::Local) => {
            let stack_frame = interpreter.call_stack.last_mut().unwrap();
            let src_register = stack_frame.registers[src_index_usize].take().expect("Non-existant register!");
            match src_register {
                Constant::Int(src_int) => {
                    interpreter.accumulator = src_int as f64;
                }
                Constant::Double(src_double) => {
                    interpreter.accumulator = src_double;
                }
                _ => panic!("Invalid move to accumulator register!"),
            }
        },

        (RegisterLocation::Global, RegisterLocation::Accumulator) => {
            interpreter.global[dst_index_usize] = Some(create_constant_double(&interpreter.accumulator));
        },
        (RegisterLocation::Global, RegisterLocation::Global) => {
            interpreter.global[dst_index_usize] = interpreter.global[src_index_usize].take();
        },
        (RegisterLocation::Global, RegisterLocation::Local) => {
            let stack_frame = interpreter.call_stack.last_mut().unwrap();
            interpreter.global[dst_index_usize] = stack_frame.registers[src_index_usize].take();
        },

        (RegisterLocation::Local, RegisterLocation::Accumulator) => {
            let stack_frame = interpreter.call_stack.last_mut().unwrap();
            stack_frame.registers[dst_index_usize] = Some(create_constant_double(&interpreter.accumulator));
        },
        (RegisterLocation::Local, RegisterLocation::Global) => {
            let stack_frame = interpreter.call_stack.last_mut().unwrap();
            stack_frame.registers[dst_index_usize] = interpreter.global[src_index_usize].take();
        },
        (RegisterLocation::Local, RegisterLocation::Local) => {
            let stack_frame = interpreter.call_stack.last_mut().unwrap();
            stack_frame.registers[dst_index_usize] = stack_frame.registers[src_index_usize].take();
        },

        _ => panic!("Invalid Mov operation!")
    }
}