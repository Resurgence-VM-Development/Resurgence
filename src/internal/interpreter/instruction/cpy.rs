use std::io::{Error, ErrorKind};

use super::super::super::{interpreter::Interpreter};
use crate::objects::constant::{Constant, create_constant_double};
use crate::objects::register::{Register, RegisterLocation, RegisterReference};

impl Interpreter {
    pub(crate) fn cpy_registers(&mut self, dst_reg: &Register, dst_reg_ref: &RegisterReference, src_reg: &Register, src_reg_ref: &RegisterReference) -> Result<(), Error> {
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
                let src_register = self.cpy_global(src_index_usize); // take the value from global memory
                match src_register {
                    Constant::Int(src_int) => {
                        self.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        self.accumulator = src_double;
                    }
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid copy to the accumulator!")),
                }
        },
            (RegisterLocation::Accumulator, RegisterLocation::Local) => {
                let src_register = self.cpy_local(src_index_usize);
                match src_register {
                    Constant::Int(src_int) => {
                        self.accumulator = src_int as f64;
                    }
                    Constant::Double(src_double) => {
                        self.accumulator = src_double;
                    }
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid copy to the accumulator!")),
                }
            },

            (RegisterLocation::Global, RegisterLocation::ConstantPool) => {
                self.global[dst_index_usize] = Some(self.cpy_constant(src_index_usize));
            }
            (RegisterLocation::Global, RegisterLocation::Accumulator) => {
                self.global[dst_index_usize] = Some(create_constant_double(&self.accumulator));
            },
            (RegisterLocation::Global, RegisterLocation::Global) => {
                self.global[dst_index_usize] = Some(self.cpy_global(src_index_usize));
            },
            (RegisterLocation::Global, RegisterLocation::Local) => {
                self.global[dst_index_usize] = Some(self.cpy_local(src_index_usize));
            },

            (RegisterLocation::Local, RegisterLocation::ConstantPool) => {
                let constant = Some(self.cpy_constant(src_index_usize));
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = constant;
            }
            (RegisterLocation::Local, RegisterLocation::Accumulator) => {
                let accumulator = Some(create_constant_double(&self.accumulator));
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = accumulator;
            },
            (RegisterLocation::Local, RegisterLocation::Global) => {
                let global_value = Some(self.cpy_global(src_index_usize));
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = global_value;
            },
            (RegisterLocation::Local, RegisterLocation::Local) => {
                let stack_frame = self.ref_stack_frame();
                stack_frame.registers[dst_index_usize] = Some(stack_frame.cpy_register(src_index_usize));
            },

            _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid cpy operation!"))
        }
        Result::Ok(())
    }
}
