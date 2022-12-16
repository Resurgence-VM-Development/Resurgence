use std::io::{Error, ErrorKind};

use super::super::super::interpreter::Interpreter;
use crate::ResurgenceError;
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};
use crate::objects::resurgence_error::ResurgenceErrorKind;

impl Interpreter {
    pub(crate) fn mov_registers(&mut self, dst_reg: &Register, dst_reg_ref: &RegisterReference, src_reg: &Register, src_reg_ref: &RegisterReference) -> Result<(), ResurgenceError> {
        // Destination register
        let Register(mut dst_index, mut dst_loc) = dst_reg; 
        let mut dst_index_usize = dst_index as usize;

        // Dereference the destination register if needed
        if *dst_reg_ref == RegisterReference::Dereference {
            Register(dst_index, dst_loc) = self.dereference_register(dst_index_usize, &dst_loc);
            dst_index_usize = dst_index as usize;
        }
        
        // Source register
        let Register(mut src_index, mut src_loc) = src_reg; 
        let mut src_index_usize = src_index as usize;

        // Dereference the source register if needed
        if *src_reg_ref == RegisterReference::Dereference {
            Register(src_index, src_loc) = self.dereference_register(src_index_usize, &src_loc);
            src_index_usize = src_index as usize;
        }
    
        match (dst_loc, src_loc) {
            (RegisterLocation::Accumulator, RegisterLocation::Global) => {
                // Start doing the move if src_register is not None
                let src_register = self.mov_global(src_index_usize); // take the value from global memory
                match src_register {
                    Constant::Int(src_int) => {
                        self.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        self.accumulator = src_double;
                    }
                    _ => {
                        let err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Invalid move to accumulator register!");
                        err.add_trace(&format!("{}: line {}", file!(), line!()));
                        return Err(err);
                    }
                }
            },
            (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                let src_register = self.mov_local(src_index_usize);
                match src_register {
                    Constant::Int(src_int) => {
                        self.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        self.accumulator = src_double;
                    },
                    _ => {
                        let err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Invalid move to accumulator register!");
                        err.add_trace(&format!("{}: line {}", file!(), line!()));
                        return Err(err);
                    }
                }
            },
    
            (RegisterLocation::Global, RegisterLocation::Accumulator) => self.global[dst_index_usize] = Some(create_constant_double(&self.accumulator)),
            (RegisterLocation::Global, RegisterLocation::Global) => self.global[dst_index_usize] = Some(self.mov_global(src_index_usize)),
            (RegisterLocation::Global, RegisterLocation::Local) => self.global[dst_index_usize] = Some(self.mov_local(src_index_usize)),
    
            (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                let accumulator = self.accumulator;
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(create_constant_double(&accumulator));
            },
            (RegisterLocation::Local, RegisterLocation::Global) => {
                let global_value = Some(self.mov_global(src_index_usize));
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = global_value;
            },
            (RegisterLocation::Local, RegisterLocation::Local) => {
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(stack_frame.mov_register(src_index_usize));
            },
            _ => {
                let err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Invalid MOV operation");
                err.add_trace(&format!("{}: line {}", file!(), line!()));
                return Err(err);
            }
        }
        Result::Ok(())
    }

    pub(crate) fn stack_mov(&mut self, dst_reg: &Register, dst_reg_ref: &RegisterReference) -> Result<(), ResurgenceError> {
        // Destination register
        let Register(mut dst_index, mut dst_loc) = dst_reg; 
        let mut dst_index_usize = dst_index as usize;

        // Dereference the destination register if needed
        if *dst_reg_ref == RegisterReference::Dereference {
            Register(dst_index, dst_loc) = self.dereference_register(dst_index_usize, &dst_loc);
            dst_index_usize = dst_index as usize;
        }

        // Check if the stack is empty
        if self.stack.is_empty() {
            let err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Stack emptry; can't move from the stack!");
            err.add_trace(&format!("{}: line {}", file!(), line!()));
            return Err(err);
        }
        let object = self.stack.remove(self.stack.len() - 1); // We do self.stack.len() - 1 to get the true index. For example, if the vector has a size of 1, the true index is 0
        match dst_loc {
            RegisterLocation::ConstantPool => {
                let err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Moving to the constant pool is forbidden!");
                err.add_trace(&format!("{}: line {}", file!(), line!()));
                return Err(err);
            },
            RegisterLocation::Accumulator => {
                match object {
                    Constant::Int(val) => self.accumulator = val as f64,
                    Constant::Double(val) => self.accumulator = val,
                    _ => {
                        let err = ResurgenceError::from(ResurgenceErrorKind::INVALID_OPERATION, "Can only write integers and doubles to the accumulator register");
                        err.add_trace(&format!("{}: line {}", file!(), line!()));
                        return Err(err);
                    }
                }
            },
            RegisterLocation::Global => self.global[dst_index_usize] = Some(object),
            RegisterLocation::Local => {
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(object);
            },
        }
        Result::Ok(())
    }
}
